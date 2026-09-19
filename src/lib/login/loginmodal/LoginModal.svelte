<script>
    import qrcode from "qrcode-generator";
    import { writeText } from "@tauri-apps/plugin-clipboard-manager";

    import ModalButton from "./ModalButton.svelte";
    import ModalInput from "./ModalInput.svelte";
    import RippleLoader from "../../common/RippleLoader.svelte";

    import {invoke} from "@tauri-apps/api/core";
    import {listen} from "@tauri-apps/api/event";
    import {openUrl} from "@tauri-apps/plugin-opener";

    export let options;

    // "choose" | "webview" | "code"
    let view = "choose";

    let offlineUsername;
    let deviceCode = null;
    let codeCopied = false;

    // Neither login command can be cancelled once invoked, so a "Cancel"
    // click or picking another login path can't stop the Rust-side future.
    // This token makes sure a late/superseded result is dropped instead of
    // silently overwriting whatever the user ended up logging in with.
    let loginAttempt = 0;

    $: deviceCodeQr = deviceCode ? buildQrSvg(deviceCode.directVerificationUri) : null;

    function buildQrSvg(text) {
        const qr = qrcode(0, "M");
        qr.addData(text);
        qr.make();
        return qr.createSvgTag({cellSize: 4, margin: 0, alt: "使用 Microsoft 登录的二维码"});
    }

    async function handleOfflineLoginClick() {
        if (offlineUsername.length > 16 || offlineUsername.length < 1) {
            alert("用户名长度必须在 1 到 16 个字符之间。");
            return;
        }

        const usernameRegex = /^[a-zA-Z0-9_]+$/;
        if (!usernameRegex.test(offlineUsername)) {
            alert("用户名只能包含字母、数字和下划线。");
            return;
        }

        options.start.account = await invoke("login_offline", {username: offlineUsername});
        options.store();
    }

    function handleMicrosoftWebviewLoginClick() {
        const attempt = ++loginAttempt;
        view = "webview";

        invoke("login_microsoft_webview")
            .then((account) => {
                if (attempt !== loginAttempt) return;
                options.start.account = account;
                options.store();
            })
            .catch((err) => {
                if (attempt !== loginAttempt) return;
                reportMicrosoftError(err);
                view = "choose";
            });
    }

    function cancelWebviewLogin() {
        loginAttempt++;
        view = "choose";
    }

    function startDeviceCodeLogin() {
        const attempt = ++loginAttempt;
        view = "code";
        deviceCode = null;

        invoke("login_microsoft_device_code")
            .then((account) => {
                if (attempt !== loginAttempt) return;
                options.start.account = account;
                options.store();
            })
            .catch((err) => {
                if (attempt !== loginAttempt) return;
                reportMicrosoftError(err);
                view = "choose";
            });
    }

    function cancelDeviceCode() {
        loginAttempt++;
        deviceCode = null;
        view = "choose";
    }

    async function copyDeviceCode() {
        try {
            await writeText(deviceCode.userCode);
            codeCopied = true;
            setTimeout(() => codeCopied = false, 1600);
        } catch (err) {
            console.error("Failed to copy code:", err);
        }
    }

    function reportMicrosoftError(err) {
        alert(
            "Microsoft 身份验证失败。\n\n" +
             err + "\n\n" +
            "如果无法解决此问题，请使用“离线登录”选项，" +
            "并尝试通过客户端内置的账户管理器登录。"
        );
    }

    listen("microsoft_device_code", (e) => {
        deviceCode = e.payload;
    });
</script>

<div class="modal">
    {#if view === "webview"}
        <div class="title">Microsoft 登录</div>

        <RippleLoader></RippleLoader>
        <div class="hint">请在弹出窗口中完成登录。</div>

        <ModalButton text="取消" primary={false} on:click={cancelWebviewLogin} />
    {:else if view === "code"}
        <div class="title">使用代码登录</div>

        {#if deviceCode}
            <div class="code-box">
                <span class="code-text">{deviceCode.userCode}</span>
                <button class="copy-button" type="button" on:click={copyDeviceCode}>{codeCopied ? "已复制" : "复制"}</button>
            </div>

            <div class="hint">
                在任意设备上打开
                <button class="inline-link" type="button" on:click={() => openUrl(deviceCode.verificationUri)}>{deviceCode.verificationUri.replace('https://', '')}</button>，并输入上方代码。
            </div>

            <div class="divider">或</div>

            <div class="qr-row">
                <div class="qr-code">{@html deviceCodeQr}</div>
                <div class="qr-copy">
                    <span class="qr-copy-title">扫码登录</span>
                    <span class="qr-copy-desc">打开已自动填入代码的页面。</span>
                </div>
            </div>

            <div class="waiting">
                <div class="waiting-dot"></div>
                <span>等待确认</span>
            </div>
        {:else}
            <div class="hint">正在请求登录代码&hellip;</div>
        {/if}

        <ModalButton text="取消" primary={false} on:click={cancelDeviceCode} />
    {:else}
        <div class="title">登录</div>

        <ModalButton text="Microsoft 登录" primary={true} on:click={handleMicrosoftWebviewLoginClick} />
        <ModalButton text="Microsoft 设备代码登录" primary={false} on:click={startDeviceCodeLogin} />

        <div class="divider">或</div>

        <ModalInput placeholder="用户名" icon="person" characterLimit={16} bind:value={offlineUsername} />
        <ModalButton text="离线登录" primary={false} on:click={handleOfflineLoginClick} />
    {/if}
</div>

<style>
    .modal {
        background-color: rgba(0, 0, 0, 0.26);
        padding: 30px;
        border-radius: 12px;
        width: 320px;
        display: flex;
        flex-direction: column;
        row-gap: 15px;
    }

    .title {
        color: white;
        font-size: 22px;
        margin: 0 auto;
        position: relative;
        width: max-content;
        margin-bottom: 40px;
    }

    .title::after {
        content: "";
        position: absolute;
        height: 5px;
        width: calc(100% - 10px);
        left: 50%;
        bottom: -20px;
        transform: translateX(-50%);
        background-color: #4677FF;
        border-radius: 5px;
    }

    .divider {
        display: flex;
        align-items: center;
        gap: 12px;
        margin: -5px 20px;
        color: rgba(255, 255, 255, .4);
        font-size: 12px;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 1px;
    }

    .divider::before,
    .divider::after {
        content: "";
        flex: 1;
        height: 1px;
        background: rgba(255, 255, 255, .15);
    }

    .hint {
        color: rgba(255, 255, 255, 0.5);
        font-size: 12px;
        text-align: center;
        line-height: 1.4;
        padding: 0 6px;
    }

    .inline-link {
        background: none;
        border: none;
        padding: 0;
        margin: 0;
        color: #4677FF;
        font-weight: 600;
        font-size: inherit;
        font-family: inherit;
        cursor: pointer;
        text-decoration: underline;
    }

    .code-box {
        display: flex;
        align-items: center;
        column-gap: 8px;
        height: 54px;
        padding: 0 8px 0 16px;
        border-radius: 8px;
        background: rgba(0, 0, 0, .26);
    }

    .code-text {
        flex: 1;
        min-width: 0;
        color: white;
        font-size: 21px;
        font-weight: 700;
        letter-spacing: 3px;
        white-space: nowrap;
    }

    .copy-button {
        flex: none;
        min-width: 76px;
        height: 38px;
        padding: 0 14px;
        border: none;
        border-radius: 6px;
        background: #4677FF;
        color: white;
        font-family: "Inter", sans-serif;
        font-size: 12px;
        font-weight: 700;
        letter-spacing: .4px;
        cursor: pointer;
        transition: ease background-color .2s;
    }

    .copy-button:hover {
        background: #3E69E2;
    }

    .qr-row {
        display: flex;
        align-items: center;
        column-gap: 14px;
    }

    .qr-code {
        flex: none;
        width: 84px;
        height: 84px;
        padding: 7px;
        border-radius: 8px;
        background: white;
        line-height: 0;
    }

    .qr-code :global(svg) {
        display: block;
        width: 100%;
        height: 100%;
    }

    .qr-copy {
        display: flex;
        flex-direction: column;
        row-gap: 6px;
    }

    .qr-copy-title {
        color: rgba(255, 255, 255, .85);
        font-size: 12px;
        font-weight: 700;
    }

    .qr-copy-desc {
        color: rgba(255, 255, 255, .5);
        font-size: 12px;
    }

    .waiting {
        display: flex;
        align-items: center;
        justify-content: center;
        column-gap: 8px;
    }

    .waiting-dot {
        flex: none;
        width: 7px;
        height: 7px;
        border-radius: 50%;
        background: #4677FF;
        animation: pulse 1.4s ease-in-out infinite;
    }

    @keyframes pulse {
        0%, 100% { opacity: .35; transform: scale(.8); }
        50% { opacity: 1; transform: scale(1); }
    }

    .waiting span {
        color: rgba(255, 255, 255, .5);
        font-size: 12px;
    }
</style>
