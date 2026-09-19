import {listenAlways} from "../integration/ws";
import {getModuleSettings} from "../integration/rest";
import {get, writable} from "svelte/store";
import type {SpaceSeperatedNamesChangeEvent} from "../integration/events";

export let spaceSeperatedNames = writable(false);

const regex = /[A-Z]?[a-z]+|[A-Z]+[0-9]+|[0-9]+|[A-Z]+(?![a-z])/g;

/**
 * Handles space seperated names if enabled.
 */
export function convertToSpacedString(name: string): string {
    return (name.match(regex) ?? [name]).join(" ");
}

/**
 * The label to render for a module, setting or mode.
 *
 * The client sends a `translatedName` alongside every `name`, because the raw
 * name has to stay English — it doubles as the config key, the command name and
 * the add-on identifier. Render this rather than `name` directly.
 *
 * Falls back to the English name, word-separated if the user asked for that.
 */
/**
 * Labels for dropdown options whose value *is* the identifier the client matches on.
 *
 * Server resource pack policies, game modes and difficulties are sent back
 * verbatim, so translating the option itself would hand the client an invalid
 * value. Only the rendered label is swapped; unknown values pass through.
 */
const OPTION_LABELS: Record<string, string> = {
    // 服务器资源包策略
    Prompt: "询问",
    Enabled: "启用",
    Disabled: "禁用",
    // 游戏模式
    Survival: "生存",
    Creative: "创造",
    Adventure: "冒险",
    Spectator: "旁观",
    // 难度
    Peaceful: "和平",
    Easy: "简单",
    Normal: "普通",
    Hard: "困难",
};

export function optionLabel(value: string): string {
    return OPTION_LABELS[value] ?? value;
}

export function displayName(value: string | { name: string; translatedName?: string }): string {
    // Callers that only hold a raw name (mode tags, list entries) get the plain
    // English, spaced if requested — there is nothing to look a translation up on.
    if (typeof value === "string") {
        return get(spaceSeperatedNames) ? convertToSpacedString(value) : value;
    }

    if (value.translatedName) {
        return value.translatedName;
    }

    return get(spaceSeperatedNames) ? convertToSpacedString(value.name) : value.name;
}

async function updateSettings() {
    const hudSettings = await getModuleSettings("HUD");
    spaceSeperatedNames.set(hudSettings.value.find(n => n.name === "SpaceSeperatedNames")?.value as boolean ?? true);
}

listenAlways("spaceSeperatedNamesChange", (e: SpaceSeperatedNamesChangeEvent) => {
   spaceSeperatedNames.set(e.value);
});
updateSettings();
