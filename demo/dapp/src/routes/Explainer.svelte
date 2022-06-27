<script>
    import { BasePage, RebaseLogo, Button } from "components";
    import { useNavigate } from "svelte-navigator";

    const navigate = useNavigate();
</script>

<BasePage>
    <div class="min-h-[577px] h-full flex flex-wrap">
        <RebaseLogo class="w-full flex items-center justify-center my-[16px]" xl4_5/>
        <div>
            <p>
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut
                vitae egestas ipsum. Quisque pharetra felis nunc, eu tristique
                metus lobortis sed. Ut viverra, quam viverra vulputate
                vulputate, tortor libero pretium quam, quis ullamcorper neque
                nulla efficitur purus. Maecenas egestas, mauris non mollis
                porttitor, orci felis consequat ante, eu pellentesque metus
                nulla at ligula. Sed mauris nunc, laoreet nec nisl ac,
                ullamcorper mollis nibh. Nunc vel varius justo. Sed cursus
                sollicitudin lorem vitae dictum
            </p>
            <br />
            <p>
                Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut
                vitae egestas ipsum. Quisque pharetra felis nunc, eu tristique
                metus lobortis sed. Ut viverra, quam viverra vulputate
                vulputate, tortor libero pretium quam, quis ullamcorper neque
                nulla efficitur purus. Maecenas egestas, mauris non mollis
                porttitor, orci felis consequat ante, eu pellentesque metus
                nulla at ligula. Sed mauris nunc, laoreet nec nisl ac,
                ullamcorper mollis nibh. Nunc vel varius justo. Sed cursus
                sollicitudin lorem vitae dictum
            </p>
        </div>
        <div class="w-full flex justify-center items-center">
            <Button
                class="w-full max-w-42 my-[16px]"
                onClick={() => navigate("/account")}
                text="Enter"
                primary
                reverse
                rounded
            />
        </div>
    </div>
</BasePage>
