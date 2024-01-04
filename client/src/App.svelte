<script lang="ts">
  // Components
  import Path from "./lib/Path.svelte";
  import SettingsButton from "./lib/SettingsButton.svelte";
  import SettingsMenu from "./lib/SettingsMenu.svelte";
  import UploadButton from "./lib/UploadButton.svelte";
  import NewFolderButton from "./lib/NewFolderButton.svelte";
  import Directory from "./lib/Directory.svelte";

  let path = "/usertal/asdasd";
  let serverIp = "";
  let elementsInPathDirectory = [{name: "one", isDirectory: true}, {name: "two.txt", isDirectory: false}, {name: "three", isDirectory: true}];

  let directoriesInCurrentPath = elementsInPathDirectory.filter((element) => {return element.isDirectory});
  let filesInCurrentPath = elementsInPathDirectory.filter((element) => {return !element.isDirectory});

  console.log(directoriesInCurrentPath);

  const isRootDirectory = () => {
    return path === "/";
  }

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

<!-- DIRECTORIES -->
{#each directoriesInCurrentPath as directory}
  <Directory directory={directory}/>
{/each}

<!-- FILES -->
{#each filesInCurrentPath as file}

{/each}

<!-- BOTTOM ROW -->
<div id="bottom-row">
  <NewFolderButton actualPath={path}/>
  <UploadButton actualPath={path}/>
</div>
