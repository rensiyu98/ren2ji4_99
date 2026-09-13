<script>
    import ToggleSetting from "../../settings/ToggleSetting.svelte";
    import SettingWrapper from "../../settings/SettingWrapper.svelte";
    import LiquidBounceAccount from "../../settings/LiquidBounceAccount.svelte";
    import Description from "../../settings/Description.svelte";
    import ButtonSetting from "../../settings/ButtonSetting.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {openUrl} from "@tauri-apps/plugin-opener";

    export let client;
    export let options;

    async function login() {
        try {
            const account = await invoke("client_account_authenticate", {
                client: client
            });
            options.premium.account = account;
            await options.store();
        } catch (error) {
            console.error("Failed to authenticate client account:", error);
            alert(`客户端账户验证失败：${error}`);
        }
    }

    async function logout() {
        options.premium.account = null
    }
</script>

<ToggleSetting
        title="跳过广告"
        disabled={!options.premium.account || !options.premium.account.premium}
        bind:value={options.premium.skipAdvertisement}
/>

{#if options.premium.account}
    <SettingWrapper title="账户信息">
        <LiquidBounceAccount account={options.premium.account} />
    </SettingWrapper>

    {#if !options.premium.account.premium}
        <Description
                description="此账户似乎没有关联高级版。请在账户管理页面进行关联。"
        />
    {/if}

    <ButtonSetting
            text="管理账户"
            on:click={() => openUrl("https://user.liquidbounce.net")}
            color="#4677FF"
    />
    <ButtonSetting
            text="退出登录"
            on:click={logout}
            color="#B83529"
    />
{:else}
    <Description
            description="升级为高级版，不仅能支持客户端的持续开发，还能获得披风并跳过启动器中的广告。"
    />

    <ButtonSetting
            text="使用 LiquidBounce 账户登录"
            on:click={login}
            color="#4677FF"
    />
{/if}