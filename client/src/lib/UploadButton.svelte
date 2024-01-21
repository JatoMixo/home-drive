<script lang="ts">
    import { invoke } from "@tauri-apps/api";

    export let actualPath = "";
    export let serverIp = "";
    let uploading = false;
    let filesUploading = undefined;

    const uploadFile = (fileToUpload) => {
        invoke("upload_file", {
            "fileUpload": {
                "name": fileToUpload.name,
                "path_to_upload": actualPath,
                "content": [65, 66, 67, 68],
            },
            "server": {
                "ip": serverIp,
                "port": 8080,
            }
        });
    }
    
    $: if (filesUploading != undefined) {
        uploadFile(filesUploading[0]);
    }

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

    #file-selector {
        padding: 14px;
    }
</style>

{#if uploading}
    <div id="file-selector" class="box">
        <input type="file" bind:files={filesUploading}/>
    </div>
{/if}

<button id="upload-button" class="box" on:click={() => uploading = !uploading}>
    <img src="/UploadSymbol.png" alt="" id="icon"/>
    <p id="upload-text">Upload</p>
</button>
