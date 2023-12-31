<script lang="ts">
  // Components
  import Path from "./lib/Path.svelte";
  import SettingsButton from "./lib/SettingsButton.svelte";
  import SettingsMenu from "./lib/SettingsMenu.svelte";
  import UploadButton from "./lib/UploadButton.svelte";
  import NewFolderButton from "./lib/NewFolderButton.svelte";
  import Directory from "./lib/Directory.svelte";
  import File from "./lib/File.svelte";

  // Tauri
  import { invoke } from "@tauri-apps/api";

  let path = "/example/path";
  let serverIp = "";
  let elementsInPathDirectory: any = [{name: "Test.txt", isDirectory: false}, {name: "TestDir", isDirectory: true}, {name: "AnotherFile.png", isDirectory: false}];

  /*invoke("get_elements_in_path", {ip: serverIp, path: path}).then((elements) => {
    elementsInPathDirectory = elements;
  });*/

  let isRootDirectory = path == "/";

  let directoriesInCurrentPath = elementsInPathDirectory.filter((element) => {return element.isDirectory});
  let filesInCurrentPath = elementsInPathDirectory.filter((element) => {return !element.isDirectory});

  let displayingSettings = false;
</script>

<style lang="scss">
  @import "styles.scss";

  #top-row {
    position: sticky;
    top: 0;

    margin-bottom: 15px;
  }

  .row {
    display: flex;
    align-items: center;

    gap: 10px;
  }

  @keyframes appear {
    from {
      opacity: 0;
    }
    to {
      opacity: 100%;
    }
  }

  #settings-menu-container {
    position: absolute;
    right: 8px;

    animation: appear 150ms;
  }

  #bottom-row {
    position: absolute;

    right: 8px;
    bottom: 10px;

    display: flex;
    align-items: center;
    gap: 5px;
  }

  #back-button {

    display: flex;
    align-items: center;

    gap: 10px;

    padding: 0px;
    width: 100%;

    border-width: 0;
    border-radius: 15px;

    background-color: $normal-background-color;
    transition: background-color 100ms;

    img {
      width: 50px;
    }

    p {
      color: white;
      text-shadow: 0 0 3px white;
      font-family: "Hack Nerd Font";
      font-size: 20px;
    }
  }

  #back-button:hover {
    background-color: $dark-background-color;
  }

  #back-button:active {
    background-color: #252525;
  }
</style>

<!-- TOP ROW -->
<div id="top-row" class="row">
  <Path path={path}/>

  <SettingsButton bind:displayingSettings={displayingSettings}/>
</div>

<!-- SETTINGS MENU -->
{#if displayingSettings}
  <div id="settings-menu-container">
    <SettingsMenu bind:ip={serverIp}/>
  </div>
{/if}

<!-- GO BACK BUTTON -->
{#if !isRootDirectory}
  <button id="back-button">
      <img src="/BackArrow.png" alt="[BACK]"/>
      <p>..</p>
  </button>
{/if}

<!-- DIRECTORIES -->
{#each directoriesInCurrentPath as directory}
  <Directory directory={directory}/>
{/each}

<!-- FILES -->
{#each filesInCurrentPath as file}
  <File file={file}/>
{/each}

<!-- BOTTOM ROW -->
<div id="bottom-row">
  <NewFolderButton actualPath={path}/>
  <UploadButton actualPath={path}/>
</div>
