<script lang="ts">
  // Components
  import Path from "./lib/Path.svelte";
  import SettingsButton from "./lib/SettingsButton.svelte";
  import SettingsMenu from "./lib/SettingsMenu.svelte";

  let path = "/usertal/asdasd";
  let elementsInPathDirectory = ["one", "two.txt", "nice"];

  const isRootDirectory = () => {
    return path === "/";
  }

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

  #settings-menu-container {
    position: absolute;
    right: 8px;
  }
</style>

<div id="top-row" class="row">
  <Path path={path}/>

  <SettingsButton/>
</div>

<div id="settings-menu-container">
  <SettingsMenu />
</div>
