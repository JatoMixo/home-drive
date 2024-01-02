<script lang="ts">
  // Components
  import Path from "./lib/Path.svelte";
  import SettingsButton from "./lib/SettingsButton.svelte";
  import SettingsMenu from "./lib/SettingsMenu.svelte";

  let path = "/usertal/asdasd";
  let serverIp = "";
  let elementsInPathDirectory = ["one", "two.txt", "nice"];

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
</style>

<div id="top-row" class="row">
  <Path path={path}/>

  <SettingsButton bind:displayingSettings={displayingSettings}/>
</div>

{#if displayingSettings}
  <div id="settings-menu-container">
    <SettingsMenu bind:ip={serverIp}/>
  </div>
{/if}
