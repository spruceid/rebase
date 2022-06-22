<script lang="ts">
  import SpinnerIcon from "../icons/SpinnerIcon.svelte";
  import "./button.scss";

  let clazz: string = "";
  export { clazz as class };
  export let primary: boolean = false;
  export let secondary: boolean = false;
  export let action: boolean = false;
  export let text: string;
  export let title: string = "";
  export let onClick: (() => void) | null;
  export let disabled: boolean = false;
  export let small: boolean = false;
  export let rounded: boolean = false;
  export let loading: boolean = false;
</script>

<button
  {disabled}
  class={`${clazz} button-container text-ellipsis overflow-hidden `}
  class:py-4={!small}
  class:py-3={small}
  class:opacity-50={disabled}
  class:cursor-not-allowed={disabled}
  class:primary-button-container={primary}
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
  {:else}
    {text}
  {/if}
</button>
