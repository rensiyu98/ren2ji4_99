<script lang="ts">
    import ButtonSetting from "../../../common/setting/ButtonSetting.svelte";
    import Tab from "../../../common/modal/Tab.svelte";
    import {addMicrosoftAccountDeviceCode, addMicrosoftAccountDeviceCodeCopyUrl} from "../../../../../integration/rest";

    let loading = false;

    async function addAccount() {
        loading = true;
        await addMicrosoftAccountDeviceCode();
    }

    async function copyUrl() {
        loading = true;
        await addMicrosoftAccountDeviceCodeCopyUrl();
    }
</script>

<Tab>
    <ButtonSetting title="关联账户" on:click={addAccount} {loading}/>
    <ButtonSetting title="复制网址" on:click={copyUrl} secondary={true} />
</Tab>
