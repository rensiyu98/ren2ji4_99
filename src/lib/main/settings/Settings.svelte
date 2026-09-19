<script>
    import {createEventDispatcher} from "svelte";
    import GeneralSettings from "./GeneralSettings.svelte";
    import PremiumSettings from "./PremiumSettings.svelte";
    import SettingsContainer from "../../settings/SettingsContainer.svelte";
    import Tabs from "../../settings/tab/Tabs.svelte";
    import MinecraftSettings from "./MinecraftSettings.svelte";

    export let client;
    export let options;
    let activeSettingsTab = "常规";

    const dispatch = createEventDispatcher();
</script>

<SettingsContainer
        title="设置"
        on:hideSettings={() => dispatch('hide')}
>
    <Tabs
            tabs={["常规", "Minecraft", "高级版"]}
            bind:activeTab={activeSettingsTab}
            slot="tabs"
    />

    {#if activeSettingsTab === "常规"}
        <GeneralSettings
                bind:options
        />
    {:else if activeSettingsTab === "Minecraft"}
        <MinecraftSettings
                bind:options
        />
    {:else if activeSettingsTab === "高级版"}
        <PremiumSettings
                {client}
                bind:options
        />
    {/if}
</SettingsContainer>