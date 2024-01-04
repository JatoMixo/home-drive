<script lang="ts">
    export let file;

    const download = () => {

    };

    const remove = () => {

    };

    const visualize = () => {

    };
</script>

<style lang="scss">
    @import "../styles.scss";

    #main-container {
        display: flex;
        align-items: center;

        background-color: $normal-background-color;

        gap: 5px;

        transition: background-color 100ms;
        padding: 5px;

        border-radius: 15px;
        border-width: 0px;

        margin-right: 0px;

        width: 100%;
    }

    #main-container:hover {
        background-color: $dark-background-color;
    }

    #main-container:active {
        background-color: #252525;
    }

    #icon {
        width: 65px;
    }

    #name {
        font-size: 20px;

        color: white;
        text-shadow: 0 0 3px white;
    }

    .action-button-icon {
        width: 50px;
    }

    #trash-icon {
        width: 30px;
    }

    .action-button {
        background-color: $normal-background-color;

        width: 60px;
        height: 60px;

        border-width: 0px;
        border-radius: 15px;
    }

    #download {
        margin-left: auto;
    }
</style>

<button id="main-container" on:click={visualize}>
    <img src="/FileSymbol.png" alt="[FILE]" id="icon"/>
    <p id="name">{file.name}</p>

    <button id="download" class="action-button" on:click={download}>
        <img src="/DownloadSymbol.png" alt="[DOWNLOAD]" class="action-button-icon"/>
    </button>
    <button id="delete" class="action-button" on:click={remove}>
        <img src="/TrashSymbol.png" alt="[DELETE]" class="action-button-icon" id="trash-icon"/>
    </button>
</button>
