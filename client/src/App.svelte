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
</style>

<Path path={path}/>
