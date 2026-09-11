# ============================================================================
# 一键发布脚本（v1.1.0）：python outputs/publish_v1.1.0.py
#
# 作用：把本地构建好的产物发布到 GitHub Release，全程不依赖 gh CLI。
#   1) 通过 `git credential fill` 向 GitHub Credential Manager 索取已缓存的
#      PAT（细粒度 token），仅在内存中使用，**不落盘**、不在脚本里硬编码。
#   2) 用 GitHub REST API 创建 Release（正文取自 outputs/release_body_v1.1.0.md）。
#   3) 上传资产。注意资产名必须用 **纯 ASCII**（如 XYplorer-ColorFilter-Tool-v1.1.0.exe）：
#      GitHub 对含中文等非 ASCII 的文件名支持不稳定，中文名会被静默剥离/乱码，
#      所以用 ASCII 文件名 + release 正文里写中文说明。
# 仓库：Marcrevo-bit/xy-colorfilter-tool，标签：v1.1.0
# ============================================================================
import urllib.request, json, os, sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
EXE = os.path.join(BASE, "src-tauri", "target", "release", "xy-colorfilter-tool.exe")
SHA = EXE + ".SHA256.txt"
BODY = os.path.join(BASE, "outputs", "release_body_v1.1.0.md")
REPO = "Marcrevo-bit/xy-colorfilter-tool"
TAG = "v1.1.0"

import subprocess
out = subprocess.run(
    ["git", "credential", "fill"],
    input="protocol=https\nhost=github.com\n",
    capture_output=True, text=True,
).stdout
pat = ""
for line in out.splitlines():
    if line.lower().startswith("password="):
        pat = line.split("=", 1)[1].strip()
if not pat:
    print("PAT empty"); sys.exit(1)
print("PAT acquired, len", len(pat))

body_text = open(BODY, "r", encoding="utf-8").read()

def api(method, url, data=None, is_upload=False):
    headers = {
        "Authorization": "Bearer " + pat,
        "Accept": "application/vnd.github+json",
        "User-Agent": "xycf-publish",
    }
    if is_upload:
        headers["Content-Type"] = "application/octet-stream"
    else:
        headers["Content-Type"] = "application/json"
    req = urllib.request.Request(url, data=data, headers=headers, method=method)
    with urllib.request.urlopen(req) as r:
        return r.getcode(), r.read().decode("utf-8", "replace")

# 1) create release
payload = json.dumps({
    "tag_name": TAG,
    "name": TAG + " · 双语界面 + 配方库扩容",
    "body": body_text,
    "draft": False,
    "prerelease": False,
}).encode("utf-8")
code, resp = api("POST", f"https://api.github.com/repos/{REPO}/releases", payload)
print("create release:", code)
rel = json.loads(resp)
rid = rel["id"]
print("release id:", rid)

# 2) upload assets (ASCII names)
def upload(path, name):
    with open(path, "rb") as f:
        data = f.read()
    url = f"https://uploads.github.com/repos/{REPO}/releases/{rid}/assets?name=" + urllib.parse.quote(name)
    code, resp = api("POST", url, data=data, is_upload=True)
    print(f"upload {name}: {code}")

upload(EXE, "XYplorer-ColorFilter-Tool-v1.1.0.exe")
upload(SHA, "XYplorer-ColorFilter-Tool-v1.1.0.exe.SHA256.txt")
print("DONE")
