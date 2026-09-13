<script>
    import {createEventDispatcher} from "svelte";
    import SettingsContainer from "../settings/SettingsContainer.svelte";
    import SelectSetting from "../settings/SelectSetting.svelte";
    import ToggleSetting from "../settings/ToggleSetting.svelte";
    import SettingWrapper from "../settings/SettingWrapper.svelte";
    import CustomModSetting from "../settings/CustomModSetting.svelte";
    import IconButtonSetting from "../settings/IconButtonSetting.svelte";
    import {invoke} from "@tauri-apps/api/core";
    import {open as dialogOpen} from "@tauri-apps/plugin-dialog";

    export let options;
    export let versionState = {
        builds: [],
        recommendedMods: [],
        customMods: [],
        currentBuild: null
    };

    const dispatch = createEventDispatcher();

    async function deleteMod(event) {
        try {
            await invoke("delete_custom_mod", {
                branch: versionState.currentBuild.branch,
                mcVersion: versionState.currentBuild.mcVersion,
                modName: `${event.detail.name}.jar`
            });
            dispatch('updateMods');
        } catch (error) {
            console.error("Failed to delete mod:", error);
            alert(`删除模组失败：${error}`);
        }
    }

    async function installMod() {
        try {
            const selected = await dialogOpen({
                directory: false,
                multiple: true,
                filters: [{ name: "", extensions: ["jar"] }],
                title: "选择要安装的自定义模组"
            });

            if (selected) {
                for (const file of selected) {
                    await invoke("install_custom_mod", {
                        branch: versionState.currentBuild.branch,
                        mcVersion: versionState.currentBuild.mcVersion,
                        path: file
                    });
                }

                dispatch('updateMods');
            }
        } catch (error) {
            console.error("Failed to install mod:", error);
            alert(`安装模组失败：${error}`);
        }
    }
</script>

<SettingsContainer
        title="选择版本"
        on:hideSettings={() => dispatch('hide')}
>
    <SelectSetting
            title="构建"
            items={[
                { value: -1, text: "最新" },
                ...versionState.builds.map(e => ({
                    value: e.buildId,
                    text: `${e.lbVersion} git-${e.commitId.substring(0, 7)} - ${e.date}`
                }))
            ]}
            bind:value={options.version.buildId}
            on:change={() => dispatch('updateData')}
    />
    <ToggleSetting
            title="显示每日构建"
            bind:value={options.launcher.showNightlyBuilds}
            disabled={false}
            on:change={() => dispatch('updateData')}
    />
    <SettingWrapper title="推荐模组">
        {#each versionState.recommendedMods as mod}
            <ToggleSetting
                    title={mod.name}
                    bind:value={mod.enabled}
                    disabled={mod.required}
                    on:change={() => dispatch('updateModStates')}
            />
        {/each}
    </SettingWrapper>
    <SettingWrapper title={`附加模组 - ${versionState.currentBuild?.subsystem ? `${versionState.currentBuild.subsystem.charAt(0).toUpperCase()}${versionState.currentBuild.subsystem.slice(1)}` : ''} ${versionState.currentBuild?.mcVersion}`}>
        <div slot="title-element">
            <IconButtonSetting
                    text="安装"
                    icon="icon-plus"
                    on:click={installMod}
            />
        </div>
        {#each versionState.customMods as mod}
            <CustomModSetting
                    title={mod.name}
                    bind:value={mod.enabled}
                    on:change={() => dispatch('updateModStates')}
                    on:delete={deleteMod}
            />
        {/each}
    </SettingWrapper>
</SettingsContainer>