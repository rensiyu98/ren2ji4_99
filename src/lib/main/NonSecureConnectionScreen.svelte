<script>
    import {createEventDispatcher} from "svelte";
    import VerticalFlexWrapper from "../common/VerticalFlexWrapper.svelte";
    import TitleBar from "../common/TitleBar.svelte";
    import Logo from "../common/Logo.svelte";
    import ButtonClose from "../common/ButtonClose.svelte";
    import {openUrl} from "@tauri-apps/plugin-opener";
    import ButtonSetting from "../settings/ButtonSetting.svelte";

    const dispatch = createEventDispatcher();
    
    function handleAllowConnection() {
        dispatch("allowNonSecure");
    }
    
    function handleCancel() {
        dispatch("cancel");
    }
</script>

<VerticalFlexWrapper blur={false}>
    <TitleBar>
        <Logo />
        <ButtonClose />
    </TitleBar>

    <div class="warning-container">
        <h1>安全警告</h1>
        <p class="warning-message">
            LiquidLauncher 无法连接到安全（HTTPS）API 端点，
            但可以连接到非安全（HTTP）端点。
        </p>

        <div class="warning-details">
            <h3>安全风险：</h3>
            <ul>
                <li>你的连接未加密，网络中的其他人可能看到你的数据</li>
                <li>发送和接收的数据可能被恶意行为者拦截或篡改</li>
                <li>通过此连接传输的登录凭据可能会泄露</li>
            </ul>

            <h3 class="recommendation">建议：</h3>
            <p>
                此问题通常由网络限制或 SSL 证书问题引起。
                我们建议使用 Cloudflare WARP 等 VPN 来建立安全连接。
            </p>
        </div>

        <div class="action-buttons">
            <ButtonSetting text="获取 Cloudflare WARP" color="#4677ff" on:click={() => openUrl('https://1.1.1.1/')}></ButtonSetting>
            <ButtonSetting text="允许非安全连接" color="#B83529" on:click={handleAllowConnection}></ButtonSetting>
            <ButtonSetting text="取消" color="#707070" on:click={handleCancel}></ButtonSetting>
        </div>
    </div>
</VerticalFlexWrapper>

<style>
    .warning-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        padding: 2rem;
        max-width: 800px;
        margin: 0 auto;
        color: white;
        flex: 1;
    }

    h1 {
        color: #ff9800;
        font-size: 2.5rem;
        margin-bottom: 1rem;
    }

    .warning-message {
        font-size: 1.2rem;
        margin-bottom: 2rem;
        text-align: center;
    }

    .warning-details {
        background-color: rgba(0, 0, 0, 0.3);
        padding: 1.5rem;
        border-radius: 6px;
        width: 100%;
        margin-bottom: 2rem;
    }

    .warning-details h3 {
        margin-bottom: 0.75rem;
        color: #ff9800;
    }

    .warning-details ul {
        margin-left: 1.5rem;
        margin-bottom: 1.5rem;
    }

    .warning-details li {
        margin-bottom: 0.5rem;
    }

    .recommendation {
        margin-top: 1rem;
    }

    .action-buttons {
        display: flex;
        gap: 1rem;
        margin-top: 1rem;
    }
</style>
