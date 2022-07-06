<script lang="ts">
  import { SpinnerIcon } from "components";
  import "./button.scss";

  let clazz: string = "";
  export { clazz as class };
  export let primary: boolean = false;
  export let reverse: boolean = false;
  export let secondary: boolean = false;
  export let action: boolean = false;
  export let text: string;
  export let title: string = "";
  export let onClick: (() => void) | null;
  export let disabled: boolean = false;
  export let small: boolean = false;
  export let rounded: boolean = false;
  export let loading: boolean = false;
  export let avatar: string | false = false;
</script>

<!-- class:py-4={!small} -->
<button
  {disabled}
  class={`${clazz} button-container text-ellipsis overflow-hidden `}
  class:py-3={small}
  class:opacity-50={disabled}
  class:cursor-not-allowed={disabled}
  class:primary-button-container={primary}
  class:reverse
  class:secondary-button-container={secondary}
  class:action-button-container={action}
  class:rounded-25={rounded}
  class:rounded-xl={!rounded}
  on:click|preventDefault={disabled || loading ? null : onClick}
  aria-label={title}
  {title}
>
  {#if loading}
    <div class="flex flex-wrap items-center justify-center">
      <SpinnerIcon class="w-6 h-6 mr-2 animate-spin" />
      {text}
    </div>
  {:else if avatar}
    <div class="flex flex-wrap items-center justify-center w-full">
      <div class="w-5 h-5 mr-1">
        <img class="w-5 h-5 rounded-full object-cover" src={avatar} alt="ENS avatar" />
      </div>
      <span class="w-9/12 sm:w-auto whitespace-nowrap sm:whitespace-normal overflow-hidden sm:overflow-visible text-ellipsis">
        {text}
      </span>
    </div>
  {:else}
    {text}
  {/if}
</button>
