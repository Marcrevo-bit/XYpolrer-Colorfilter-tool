import hashlib, glob, os, sys

base = os.path.dirname(os.path.abspath(__file__))

# 刚构建出的 exe（cargo build --release 产物）
exe = os.path.join(base, "src-tauri", "target", "release", "xy-colorfilter-tool.exe")
if not os.path.exists(exe):
    print("未找到构建产物：", exe)
    print("请先运行 `cd src-tauri && cargo build --release`")
    sys.exit(1)

data = open(exe, 'rb').read()
sha = hashlib.sha256(data).hexdigest()
print("SHA256:", sha)
with open(exe + ".SHA256.txt", "w", encoding="utf-8") as f:
    f.write(sha + "  " + os.path.basename(exe) + "\n")
print("wrote", os.path.basename(exe) + ".SHA256.txt")

# 找 Tauri Brotli codegen 资源并解压，验证新 UI 明文已嵌入
try:
    import brotli
except Exception as e:
    print("brotli module missing:", e)
    sys.exit(0)

cands = glob.glob(os.path.join(
    base, "src-tauri", "target", "release", "build",
    "xy-colorfilter-tool-*", "out", "tauri-codegen-assets", "*.html"))
print("codegen assets found:", len(cands))
found = 0
needles = ["Marcrevo", "内置配方库", "时间热度", "属性状态", "超长文件名",
           "符号链接目录", "presetLib", "季度销售报表", "lenT: >= 64"]
for c in cands:
    raw = open(c, 'rb').read()
    try:
        dec = brotli.decompress(raw).decode('utf-8', 'replace')
    except Exception as e:
        print("decode fail", c, e)
        continue
    for n in needles:
        if n in dec:
            found += 1
            print("  OK embed:", n)
print("embedded needles matched:", found, "/", len(needles))
