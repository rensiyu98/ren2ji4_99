<script>
    import SelectSetting from "../../settings/SelectSetting.svelte";
    import FileSelectorSetting from "../../settings/FileSelectorSetting.svelte";
    import DirectorySelectorSetting from "../../settings/DirectorySelectorSetting.svelte";
    import RangeSetting from "../../settings/RangeSetting.svelte";
    import ToggleSetting from "../../settings/ToggleSetting.svelte";
    import ButtonSetting from "../../settings/ButtonSetting.svelte";
    import LauncherVersion from "../../settings/LauncherVersion.svelte";
    import {onMount} from "svelte";
    import {invoke} from "@tauri-apps/api/core";

    export let options;

    let launcherVersion = "";
    let defaultDataFolder = "";
    let systemMemory = options.start.memory;

    async function clearData() {
        try {
            await invoke("clear_data", { options });
            alert("数据已清除。");
        } catch (error) {
            console.error("Failed to clear data:", error);
            alert(`清除数据失败：${error}`);
        }
    }

    async function logout() {
        try {
            await invoke("logout", { accountData: options.start.account });
            options.start.account = null;
            await options.store();
        } catch (error) {
            console.error("Logout failed:", error);
            alert("退出登录失败，请重试。");
        }
    }

    function resetJavaDistribution() {
        if (options.start.javaDistribution.type === "custom") {
            options.start.javaDistribution.value = "";
        } else if (options.start.javaDistribution.type === "manual") {
            options.start.javaDistribution.value = "temurin";
        }
    }

    onMount(async () => {
        const [version, folder, memory] = await Promise.all([
            invoke("get_launcher_version"),
            invoke("default_data_folder_path"),
            invoke("sys_memory"),
        ]);

        systemMemory = memory;
        launcherVersion = version;
        defaultDataFolder = folder;
    });
</script>

<SelectSetting
    title="JVM 发行版"
    items={[
        { value: "automatic", text: "自动" },
        { value: "manual", text: "手动" },
        { value: "custom", text: "自定义" },
    ]}
    on:change={resetJavaDistribution}
    bind:value={options.start.javaDistribution.type}
/>

{#if options.start.javaDistribution.type === "manual"}
    <SelectSetting
        title="发行版"
        items={[
            { value: "temurin", text: "Eclipse Temurin" },
            { value: "graalvm", text: "GraalVM" },
            { value: "zulu", text: "Azul Zulu" },
        ]}
        bind:value={options.start.javaDistribution.value}
    />
{/if}

{#if options.start.javaDistribution.type === "custom"}
    <FileSelectorSetting
        title="自定义 JVM 路径"
        placeholder="选择 Java 包装器位置"
        bind:value={options.start.javaDistribution.value}
        filters={[{ name: "java", extensions: [] }]}
        windowTitle="选择自定义 Java 包装器"
    />
{/if}

<DirectorySelectorSetting
    title="数据目录"
    placeholder={defaultDataFolder}
    bind:value={options.start.customDataPath}
    windowTitle="选择自定义数据目录"
/>

<RangeSetting
    title="内存"
    min={2048}
    max={systemMemory}
    bind:value={options.start.memory}
    valueSuffix=" MB"
    step={128}
/>

<RangeSetting
    title="并发下载数"
    min={1}
    max={50}
    bind:value={options.launcher.concurrentDownloads}
    valueSuffix=" 个连接"
    step={1}
/>

<ToggleSetting
    title="保持启动器运行"
    disabled={false}
    bind:value={options.launcher.keepLauncherOpen}
/>

<ButtonSetting
    text="退出 Minecraft 账户"
    on:click={logout}
    color="#4677FF"
/>

<ButtonSetting text="清除数据" on:click={clearData} color="#B83529" />

<LauncherVersion version={launcherVersion} />
