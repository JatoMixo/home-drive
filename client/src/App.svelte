<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let directories = "";
  
  async function getDirectories() {
    directories = await invoke("get_directories", {path: "/"});
  }

  getDirectories();
</script>

<style lang="scss">
  @import "styles.scss";
</style>

<h1>Hello!</h1>
<h1>List of directories: {directories}</h1>
