<script>
    import {createEventDispatcher} from "svelte";
    import SettingsContainer from "../settings/SettingsContainer.svelte";
    import Description from "../settings/Description.svelte";
    import ButtonSetting from "../settings/ButtonSetting.svelte";
    import {openUrl} from "@tauri-apps/plugin-opener";

    const dispatch = createEventDispatcher();
</script>

<SettingsContainer
        title="欢迎使用 LiquidBounce！"
        on:hideSettings={() => dispatch('hide')}
>
    <Description
            description="首次启动时，或有 LiquidBounce 新更新可用时，会弹出一个下载窗口。这是我们为客户端赚取广告收入的方式。如果你不想看到广告，可以购买 LiquidBounce 高级版。你也可以查看快速入门指南，了解使用 LiquidBounce 的技巧。"
    />

    <ButtonSetting
            text="快速入门指南"
            on:click={() => openUrl("https://liquidbounce.net/docs/get-started")}
            color="#B83529"
    />

    <ButtonSetting
            text="继续"
            on:click={() => dispatch('continue')}
            color="#4677FF"
    />
</SettingsContainer>