<script lang="ts">

</script>

<style lang="scss">
    @import "../styles.scss";

    #upload-button {
        display: flex;
        align-items: center;
        gap: 5px;

        padding: 5px;
    }

    #icon {
        width: 40px;

        margin: 0;
    }

    #upload-text {
        font-size: 20px;
        font-family: "Hack Nerd Font";

        margin: 0;
        margin-right: 6px;
    }
</style>

<button id="upload-button" class="box">
    <img src="/UploadSymbol.png" alt="" id="icon"/>
    <p id="upload-text">Upload</p>
</button>
