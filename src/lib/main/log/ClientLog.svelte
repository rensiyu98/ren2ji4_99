<script>
    import { fly } from "svelte/transition";
    import VirtualList from "./VirtualList.svelte";
    import { createEventDispatcher } from "svelte";
    import { confirm } from "@tauri-apps/plugin-dialog";
    import ToggleSetting from "../../settings/ToggleSetting.svelte";
    import ButtonSetting from "../../settings/ButtonSetting.svelte"
    import LogMessage from "./LogMessage.svelte";

    export let messages;

    let autoScroll = true;

    const dispatch = createEventDispatcher();

    async function handleUploadSetting(e) {
        const log = messages.join("");

        if (await confirm("本次会话的完整日志将被上传。它可能包含私密聊天消息等敏感信息。确定要继续吗？") !== true) {
            return;
        }

        const response = await fetch("https://paste.ccbluex.net/api.php", {
            body: `content=${log}`,
            headers: {
                "Content-Type": "application/x-www-form-urlencoded",
            },
            method: "POST"
        });

        const responseData = await response.text();

        if (response.status !== 200) {
            alert(`上传日志失败：${responseData}`);
            return;
        }

        prompt("你的日志可在以下网址获取：", responseData);
    }
</script>

<div class="log" transition:fly={{ y: -10, duration: 200 }}>
    <div class="header">
        <div class="title">客户端日志</div>
        <button class="button-hide" on:click={() => dispatch("hideClientLog")}>
            <img class="icon" src="img/icon/icon-button-close.svg" alt="隐藏">
        </button>
    </div>

    <div class="output">
        <VirtualList items={messages} let:item {autoScroll}>
            <LogMessage text={item} />
        </VirtualList>
    </div>

    <div class="settings">
        <ButtonSetting text="上传日志" color="#4677FF" on:click={handleUploadSetting}></ButtonSetting>
        <ToggleSetting title="自动滚动" disabled={false} bind:value={autoScroll} />
    </div>
</div>

<style>
    .log {
        width: calc(100% - 150px);
        height: calc(100% - 150px);
        position: fixed;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        background-color: rgba(0, 0, 0, 0.58);
        backdrop-filter: blur(10px);
        padding: 25px;
        border-radius: 6px;
        z-index: 1000;
        display: flex;
        flex-direction: column;
        box-shadow: 0 0 10px rgba(0, 0, 0, 0.5);
    }

    .settings {
        display: flex;
        gap: 20px;
        align-items: center;
    }

    .output {
        flex: 1;
        overflow: hidden;
        margin-bottom: 10px;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        margin-bottom: 20px;
    }

    .title {
        font-size: 16px;
        color: white;
    }

    .button-hide {
        background-color: transparent;
        border: none;
        cursor: pointer;
    }
</style>
