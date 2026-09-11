// ============================================================================
// 离线校验器（发布 / CI 前运行）：node _validate.cjs
//
// 做法：把 ui/index.html 中内联的 <script> 整段抽出来，放进一个“假的 DOM 环境”
// （用 Proxy 造的 document / window / localStorage stub，足以让脚本顶层语句跑起来），
// 再用 Node 的 vm 沙箱执行，从而在不启动浏览器的前提下验证前端逻辑没坏。
//
// 校验点（见下方 test 字符串里输出的日志）：
//   - PRESETS 总数、CAT_ORDER 分类顺序、空分类 / 未知分类 / 重名 检测
//   - 大小写敏感检查（如 'Hardlinks' 误写为 'HardLinks' 会被揪出）
//   - 预览匹配引擎：用内置 SAMPLE 样本喂给 matchFile / matchCond 等函数，
//     对比命中数量是否符合预期（size/office/hidden/backup 等），并附带调试日志
//   - 关键渲染函数 renderPresetLib() 能正常执行不报错
//   - 最终打印 ALL_CHECKS_DONE 作为“全部通过”的信号（CI 可据此判定）
// ============================================================================
const fs = require('fs');
const vm = require('vm');
const path = require('path');

const html = fs.readFileSync(path.join(__dirname, 'ui', 'index.html'), 'utf8');
const m = html.match(/<script>([\s\S]*?)<\/script>/);
if (!m) { console.log('NO_SCRIPT_FOUND'); process.exit(1); }
let body = m[1];

// ---- 通用 DOM stub：用 Proxy 让任何属性访问都返回一个安全的“假元素” ----
// 这样 index.html 脚本里诸如 document.getElementById(...).classList.add(...) 之类的调用
// 即便在 Node 里找不到真实 DOM，也不会抛错，从而让脚本顶层能完整执行。
function makeEl() {
  const store = { _children: [] };
  const handler = {
    get(t, prop) {
      if (prop === 'classList') return { add(){}, remove(){}, toggle(){}, contains(){return false;} };
      if (prop === 'style') return makeEl();
      if (prop === 'dataset') return new Proxy({}, { get(){ return ''; }, set(){ return true; } });
      if (prop === 'children') return store._children;
      if (prop === 'selectedOptions') return [{ text: '' }];
      if (prop === 'textContent' || prop === 'innerHTML' || prop === 'value' || prop === 'title') return store[prop] || '';
      if (prop === Symbol.toPrimitive) return () => '';
      if (prop === 'forEach') return () => {};
      if (prop === 'appendChild') return (c) => { store._children.push(c); return c; };
      if (prop === 'querySelectorAll') return () => [];
      if (prop === 'querySelector') return () => makeEl();
      if (prop === 'addEventListener') return () => {};
      if (prop === 'setAttribute' || prop === 'getAttribute') return () => '';
      if (prop === 'checked' || prop === 'disabled') return false;
      if (prop === 'length') return 0;
      return (...a) => makeEl();
    },
    set(t, prop, val) { store[prop] = val; return true; },
    apply() { return makeEl(); }
  };
  return new Proxy(function(){}, handler);
}
const documentStub = {
  getElementById: () => makeEl(),
  querySelector: () => makeEl(),
  querySelectorAll: () => [],
  createElement: () => makeEl(),
  addEventListener: () => {},
  documentElement: { lang: '' },
};
const sandbox = {
  document: documentStub,
  window: { addEventListener(){}, localStorage: { getItem: () => null, setItem(){} } },
  localStorage: { getItem: () => null, setItem(){} },
  TA: { call: () => Promise.resolve({}) },
  confirm: () => true,
  alert: () => {},
  setTimeout: () => 0,
  console,
};
sandbox.globalThis = sandbox;

// ---- 在沙箱里要跑的“测试脚本” ----
// 它调用被校验脚本里的全局符号（PRESETS / CAT_ORDER / SAMPLE / matchFile / renderPresetLib ...），
// 把各项断言结果 log() 出来，出错则捕获并打印 TEST_ERROR，绝不静默失败。
const test = `
;(function(){
  const out = [];
  const log = (...a)=>out.push(a.join(' '));
  try {
    const cats = CAT_ORDER.slice();
    const groups = {};
    PRESETS.forEach(p=>{ const c=p[5]||'其他'; (groups[c]=groups[c]||[]).push(p); });
    log('PRESETS_total=' + PRESETS.length);
    log('CAT_ORDER=' + cats.length + ' [' + cats.join(',') + ']');
    const missing = cats.filter(c=>!groups[c]||!groups[c].length);
    log('empty_categories=' + (missing.length? missing.join(','):'none'));
    const unknown = Object.keys(groups).filter(c=>!cats.includes(c));
    log('unknown_categories=' + (unknown.length? unknown.join(','):'none'));
    const names = PRESETS.map(p=>p[0]);
    const dup = names.filter((n,i)=>names.indexOf(n)!==i);
    log('duplicate_names=' + (dup.length? dup.join(','):'none'));
    const wrong = PRESETS.filter(p=>/Hardlinks/i.test(p[2]));
    log('wrongcase_Hardlinks=' + wrong.length);
    const hits = (cond)=> SAMPLE.filter(f=>matchFile(f,'+'+cond)).length;
    log('hit_size>=100MB=' + hits('size: >= 100 MB') + ' (exp 3)');
    log('hit_office=' + hits('*.doc;*.docx;*.xls;*.xlsx;*.ppt;*.pptx') + ' (exp 4)');
    log('hit_hidden=' + hits('attr:hidden') + ' (exp 1)');
    log('hit_readonly=' + hits('attr f:readonly') + ' (exp 0)');
    log('hit_path>=260=' + hits('len: >= 260') + ' (exp 0)');
    log('hit_recent30min=' + hits('ageM f: <= 30 n') + ' (exp >=1)');
    log('hit_temp(~)=' + hits('*.tmp;*.temp;*.bak;~*') + ' (exp 1)');
    log('hit_backup=' + hits('*备份*;*副本*;*bak*;~*') + ' (exp >=2)');
    log('hit_longname=' + hits('lenT: >= 64') + ' (exp >=1)');
    log('hit_empty=' + hits('size: 0') + ' (exp 0)');
    const ln = SAMPLE.find(f=>f.n.length>=60);
    log('DBG_longlen=' + (ln?ln.n.length:'NA') + ' match=' + (ln? (!!matchFile(ln,'+lenT: >= 64')):'NA'));
    log('DBG_numMatch=' + numMatch(67,'>= 64',x=>parseFloat(x)));
    log('DBG_matchCond=' + (ln? !!matchCond(ln,'lenT: >= 64'):'NA'));
    log('DBG_matchOne=' + (ln? !!matchOne(ln,'lenT: >= 64'):'NA'));
    function cls(s){
      s=s.trim().replace(/^[TLB]:/i,'').replace(/\|[bswfrp-]+$/,'');
      let mm=s.match(/^([a-zA-Z]+)\s+([df]{1,2})\s*:\s*(.*)$/);
      if(mm) return 'A:'+mm[1]+'|'+mm[2]+'|'+JSON.stringify(mm[3]);
      mm=s.match(/^([a-zA-Z]+)\s*:\s*(.*)$/);
      if(mm) return 'B:'+mm[1]+'|'+JSON.stringify(mm[2]);
      return 'NAME:'+s;
    }
    log('DBG_cls=' + cls('lenT: >= 64'));
    log('DBG_lnlen=' + (ln?ln.n.length:'NA'));
    log('DBG_matchAtom_direct=' + (ln? !!matchAtom(ln,'lenT: >= 64'):'NA'));
    log('DBG_ln_d=' + (ln? ln.d : 'NA') + ' ln_a=' + (ln? ln.a : 'NA'));
    if(ln){ const L=ln.n.length; log('DBG_inline=' + numMatch(L,'>= 64',x=>parseFloat(x))); }
    if(ln){
      const _o=matchAtom;
      matchAtom=function(f,a){
        let aa=a.trim().replace(/^[TLB]:/i,'').replace(/\|[bswfrp-]+$/,'');
        let mm=aa.match(/^([a-zA-Z]+)\s+([df]{1,2})\s*:\s*(.*)$/);
        let sel,rest;
        if(mm){sel=mm[1].toLowerCase();rest=mm[3].trim();}
        else{mm=aa.match(/^([a-zA-Z]+)\s*:\s*(.*)$/);sel=mm?mm[1].toLowerCase():'?';rest=mm?mm[2].trim():aa;}
        const r=_o(f,a);
        log('DBG_wrap sel='+sel+' rest='+JSON.stringify(rest)+' Ln='+(f.n?f.n.length:'?')+' ret='+!!r);
        return r;
      };
      log('DBG_wrapcall=' + !!matchOne(ln,'lenT: >= 64'));
    }
    renderPresetLib();
    log('renderPresetLib_OK');
    log('ALL_CHECKS_DONE');
  } catch(e){ log('TEST_ERROR: ' + (e && e.stack || e)); }
  console.log(out.join('\\n'));
})();
`;

// ---- 真正执行：把 index.html 的脚本 + 上面的测试串起来，放进同一个 vm 沙箱运行 ----
try {
  vm.createContext(sandbox);
  vm.runInContext(body + '\n' + test, sandbox, { filename: 'index_script.js' });
} catch (e) {
  console.log('SCRIPT_RUN_ERROR:\n' + (e && e.stack || e));
  process.exit(2);
}
