<script>
    import ToggleSetting from "../../settings/ToggleSetting.svelte";
    import DirectorySelectorSetting from "../../settings/DirectorySelectorSetting.svelte";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";
    import Description from "../../settings/Description.svelte";

    export let options;

    let installation = null;

    async function getInstallation() {
        try {
            installation = await invoke("get_minecraft_installation", {
                customPath: options.start.installation.customPath || null 
            });
        } catch (e) {
            console.error("Failed to get vanilla status:", e);
        }
    }

    onMount(getInstallation);
    $: if (options.start.installation.customPath !== undefined) {
        getInstallation();
    }
</script>

<Description description="这将允许你使用另一个 Minecraft 安装中的存档、资源包和光影包。" />

<DirectorySelectorSetting
        title="Minecraft 目录"
        placeholder={installation?.path || "自动检测"}
        bind:value={options.start.installation.customPath}
        windowTitle="选择 Minecraft 目录"
/>

{#if installation}
    <ToggleSetting
        title="关联存档 ({installation.saves_count})"
        disabled={false}
        bind:value={options.start.installation.useVanillaSaves}
    />

    <ToggleSetting
        title="关联资源包 ({installation.resource_packs_count})"
        disabled={false}
        bind:value={options.start.installation.useVanillaResourcePacks}
    />

    <ToggleSetting
        title="关联光影包 ({installation.shader_packs_count})"
        disabled={false}
        bind:value={options.start.installation.useVanillaShaderPacks}
    />
{:else}
    <Description description="未找到原版 Minecraft 安装。"/>
{/if}
