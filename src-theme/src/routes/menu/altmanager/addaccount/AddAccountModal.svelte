<script lang="ts">
    import Modal from "../../common/modal/Modal.svelte";
    import TheAlteningAccountTab from "./TheAlteningAccountTab.svelte";
    import Tabs from "../../common/modal/Tabs.svelte";
    import CrackedAccountTab from "./CrackedAccountTab.svelte";
    import SessionAccountTab from "./SessionAccountTab.svelte";
    import {setItem} from "../../../../integration/persistent_storage";
    import CredentialsTab from "./microsoft/CredentialsTab.svelte";
    import WebViewTab from "./microsoft/WebViewTab.svelte";
    import DeviceCodeTab from "./microsoft/DeviceCodeTab.svelte";

    export let visible: boolean;

    const tabs = [
        {
            title: "Microsoft",
            icon: "icon-microsoft.svg",
            content: [
                {
                    title: "网页视图",
                    content: WebViewTab
                },
                {
                    title: "设备代码",
                    content: DeviceCodeTab
                },
                {
                    title: "凭据",
                    content: CredentialsTab
                }
            ]
        },
        {
            title: "TheAltening",
            icon: "icon-thealtening.svg",
            content: TheAlteningAccountTab
        },
        {
            title: "离线",
            icon: "icon-cracked.png",
            content: CrackedAccountTab
        },
        {
            title: "会话",
            icon: "icon-session.svg",
            content: SessionAccountTab
        }
    ];

    let activeTab = parseInt(localStorage.getItem("altmanager_add_account_active_tab") ?? "0");

    async function handleChangeTab(nextActiveTab: number) {
        activeTab = nextActiveTab;
        await setItem("altmanager_add_account_active_tab", nextActiveTab.toString());
    }
</script>

<Modal title="添加账户" bind:visible={visible}>
    <Tabs {tabs} {activeTab} onChangeTab={handleChangeTab}/>
</Modal>
