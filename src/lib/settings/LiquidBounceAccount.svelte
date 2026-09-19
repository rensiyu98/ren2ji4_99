<script>
    export let account;
</script>

<div class="liquidbounce-account">
    <div class="stat">
        <div class="stat-title">用户名</div>
        <div class="stat-value">{account.nickname}</div>
    </div>
    <div class="stat">
        <div class="stat-title">捐赠者特权</div>
        <div class="stat-value">
            {#if account.premium}
                <span style="color: #60B675">已激活</span>
            {:else}
                <span style="color: #FE4C2E">未激活</span>
            {/if}
        </div>
    </div>
</div>

<style>
    .stat {
        color: white;
        display: flex;
    }

    .stat-title {
        margin-right: 5px;
    }

    .stat-title::after {
        content: ":";
    }
</style>