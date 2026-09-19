/*
 * This file is part of LiquidBounce (https://github.com/CCBlueX/LiquidBounce)
 *
 * Copyright (c) 2015 - 2026 CCBlueX
 *
 * LiquidBounce is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * LiquidBounce is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with LiquidBounce. If not, see <https://www.gnu.org/licenses/>.
 */
package net.ccbluex.liquidbounce.integration.interop.protocol.rest.v1.client

import com.google.gson.JsonObject
import net.ccbluex.liquidbounce.lang.moduleNameTranslation
import net.ccbluex.liquidbounce.lang.settingNameTranslation

/**
 * Adds a `translatedName` next to every `name` in a serialized value group.
 *
 * Settings and modules are stored and referenced by their English names — they
 * double as config keys, command names and add-on identifiers — so the theme is
 * handed the localized label alongside rather than being asked to translate.
 *
 * Groups nest as `name`/`value` pairs, hence the recursion. The outermost object
 * is the module itself; everything below it is a setting, group or mode.
 */
internal fun JsonObject.withTranslatedNames(isModule: Boolean = false): JsonObject {
    get("name")?.takeIf { it.isJsonPrimitive }?.asString?.let {
        addProperty(
            "translatedName",
            if (isModule) moduleNameTranslation(it) else settingNameTranslation(it)
        )
    }

    get("value")?.takeIf { it.isJsonArray }?.asJsonArray?.forEach { element ->
        if (element.isJsonObject) {
            element.asJsonObject.withTranslatedNames()
        }
    }

    return this
}
