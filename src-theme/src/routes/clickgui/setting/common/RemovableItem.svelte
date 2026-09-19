<script lang="ts">
    import {createEventDispatcher} from "svelte";

    const dispatch = createEventDispatcher();
</script>

<div class="removable-item">
    <slot/>

    <button class="button-remove" title="移除" on:click={() => dispatch("remove")}>
        <img src="img/clickgui/icon-cross.svg" alt="移除">
    </button>
</div>

<style lang="scss">
  .removable-item {
    display: grid;
    grid-template-columns: 1fr max-content;
    column-gap: 5px;
    align-items: center;
    padding-right: 8px;
  }

  .button-remove {
    background-color: transparent;
    border: none;
    cursor: pointer;
  }
</style>
