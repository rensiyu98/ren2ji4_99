<script>
    import {fly} from "svelte/transition";

    const facts = [
        {
            title: "张扬作弊还是低调作弊，你说了算！",
            description: "LiquidBounce 内置大量张扬与低调的作弊功能，是全能客户端的不二之选。",
        },
        {
            title: "多版本支持",
            description: "借助内置的版本切换器，无需重启游戏即可在不同 Minecraft 版本间轻松切换！",
        },
        {
            title: "多服务器支持",
            description: "LiquidBounce 为众多不同服务器提供功能与反作弊绕过。"
        },
        {
            title: "自动配置系统",
            description: "加入知名服务器时，LiquidBounce 的自动配置系统会自动为你应用最佳设置。",
        },
        {
            title: "随心定制",
            description: "LiquidBounce 的界面完全可定制，你可以按自己的喜好更换整套界面。",
        },
        {
            title: "累计下载 600 万次",
            description:
                "LiquidBounce 是有史以来最受欢迎的作弊客户端之一。",
        },
        {
            title: "免费且开源",
            description: "LiquidBounce 的源代码公开可查。",
        },
        {
            title: "ScriptAPI",
            description:
                "LiquidBounce 的 Script API 让用户可以编写自己的模块和命令。",
        },
    ];

    let currentFact = 0;

    function handlePrevClick(e) {
        if (currentFact <= 0) {
            currentFact = facts.length - 1;
        } else {
            currentFact--;
        }
    }

    function handleNextClick(e) {
        if (currentFact >= facts.length - 1) {
            currentFact = 0;
        } else {
            currentFact++;
        }
    }

    setInterval(() => {
        handleNextClick();
    }, 5000);
</script>

<div class="wrapper">
    {#key currentFact}
        <div class="fact" in:fly={{duration: 200, x: 50}} out:fly={{duration: 200, x: -50}}>
            <div class="title">{facts[currentFact].title}</div>
            <div class="description">{facts[currentFact].description}</div>
            <div class="buttons-wrapper">
                <button
                    type="button"
                    class="button-switch-fact"
                    on:click={handlePrevClick}
                >
                    <img src="img/icon/icon-prev.svg" alt="上一个" />
                </button>
                <button
                    type="button"
                    class="button-switch-fact"
                    on:click={handleNextClick}
                >
                    <img src="img/icon/icon-next.svg" alt="下一个" />
                </button>
            </div>
        </div>
    {/key}
</div>

<style>
    .wrapper {
        position: relative;
    }

    .fact {
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);
        width: 400px;
        display: flex;
        flex-direction: column;
        row-gap: 10px;
    }

    .title {
        font-size: 40px;
        color: white;
        font-weight: 600;
    }

    .description {
        font-size: 18px;
        color: rgba(255, 255, 255, 0.5);
    }

    .buttons-wrapper {
        display: flex;
        column-gap: 10px;
    }

    .button-switch-fact {
        height: 44px;
        width: 44px;
        border-radius: 50%;
        border: solid 2px rgba(255, 255, 255, 0.5);
        background-color: transparent;
        display: flex;
        align-items: center;
        justify-content: center;
        transition: ease background-color 0.2s;
        cursor: pointer;
    }

    .button-switch-fact:hover {
        background-color: rgba(255, 255, 255, 0.1);
    }
</style>
