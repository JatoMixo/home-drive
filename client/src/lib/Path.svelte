<script lang="ts">
    export let path = "";
</script>

<style lang="scss">

    $red-color: #EF3054;
    $dark-background-color: #090909;

    #path-container {
        border: solid 4px $red-color;
        box-shadow: 0 0 10px $red-color;
        border-radius: 15px;

        background-color: $dark-background-color;

        padding: 10px;
    }

    #path-text {
        margin: 0px;

        font-size: 20px;
        color: $red-color;
        text-shadow: 0 0 3px $red-color;
    }
</style>

<div id="path-container">
    <p id="path-text">{path}</p>
</div>
