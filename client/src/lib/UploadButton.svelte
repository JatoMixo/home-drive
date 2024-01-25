<script>
    import { invoke } from "@tauri-apps/api";

    export let actualPath = "";
    export let serverIp = "";
    export let filesInCurrentDirectory = [];

    let uploading = false;
    let filesUploading = undefined;

    const getFileContent = async (file) => {
        let buffer = await file.arrayBuffer();
        let byteArray = Array.from(new Uint8Array(buffer));

        return byteArray;
    }

    const uploadFile = (fileToUpload) => {
        if (!fileToUpload) {
            return;
        }

        getFileContent(fileToUpload).then((fileContent) => {
            invoke("upload_file", {
                "fileUpload": {
                    "name": fileToUpload.name,
                    "path_to_upload": actualPath,
                    "content": fileContent,
                },
                "server": {
                    "ip": serverIp,
                    "port": 8080,
                }
            });

            uploading = false;
            filesUploading = undefined;

            filesInCurrentDirectory.push(fileToUpload.name);
            filesInCurrentDirectory = filesInCurrentDirectory;
        });
    }
    
    $: if (filesUploading != undefined) {
        uploadFile(filesUploading[0]);
    }
</script>

<style lang="scss">
    @import "../scss/styles.scss";
    @import "../scss/box.scss";
    @import "../scss/button.scss";

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
