<script>
    import { Link } from "svelte-navigator";
</script>

<div class="viewer">
    <h3 class="inner-center"><img alt="Rebase Logo" src="assets/rebase_icon.svg" /> Rebase</h3>
    <p class="inner-center">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut vitae
        egestas ipsum. Quisque pharetra felis nunc, eu tristique metus lobortis
        sed. Ut viverra, quam viverra vulputate vulputate, tortor libero pretium
        quam, quis ullamcorper neque nulla efficitur purus. Maecenas egestas,
        mauris non mollis porttitor, orci felis consequat ante, eu pellentesque
        metus nulla at ligula. Sed mauris nunc, laoreet nec nisl ac, ullamcorper
        mollis nibh. Nunc vel varius justo. Sed cursus sollicitudin lorem vitae
        dictum
    </p>
    <p class="inner-center">
        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut vitae
        egestas ipsum. Quisque pharetra felis nunc, eu tristique metus lobortis
        sed. Ut viverra, quam viverra vulputate vulputate, tortor libero pretium
        quam, quis ullamcorper neque nulla efficitur purus. Maecenas egestas,
        mauris non mollis porttitor, orci felis consequat ante, eu pellentesque
        metus nulla at ligula. Sed mauris nunc, laoreet nec nisl ac, ullamcorper
        mollis nibh. Nunc vel varius justo. Sed cursus sollicitudin lorem vitae
        dictum
    </p>
    <p class="inner-center"><Link to="/account">Enter</Link></p>
</div>

<style>
    .viewer {
        height: 70vh;
        width: 75vh;
        background-color: white;
    }
    .inner-center {
        display: flex;
        justify-content: center;
        align-items: center;
        margin-left: 5vh;
        margin-right: 5vh;
    }
</style>
