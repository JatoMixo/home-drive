<script lang="ts">
  // Components
  import Path from "./lib/Path.svelte";

  let path = "/usertal/asdasd";
  let elementsInPathDirectory = ["one", "two.txt", "nice"];

  const isRootDirectory = () => {
    return path === "/";
  }

</script>

<style lang="scss">
  @import "styles.scss";

  #path {
    position: sticky;
    top: 0;
  }
</style>

<div id="path">
  <Path path={path}/>
</div>
