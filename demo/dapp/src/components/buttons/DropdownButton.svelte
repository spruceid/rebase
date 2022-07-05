<script lang="ts">
  import { onMount } from "svelte";
import SpinnerIcon from "../icons/SpinnerIcon.svelte";
  import "./button.scss";

  let clazz: string = "";
  export { clazz as class };
  export let primary: boolean = false;
  export let secondary: boolean = false;
  export let text: string;
  export let title: string = "";
  export let disabled: boolean = false;
  export let small: boolean = false;
  export let ml: boolean = false;
  export let loading: boolean = false;

  let showDropdown = false;
  let dropdownRef = null;

  export const closeDropdown = () => (showDropdown = false);

  onMount(() => {
    const handleClickOut = (e) => {
      if (showDropdown && !dropdownRef?.contains(e.target)) {
        showDropdown = false;
      }
    };

    const handleESC = (e) => {
      if (showDropdown && e.key === "Escape") {
        showDropdown = false;
      }
    };

    document.addEventListener("click", handleClickOut, false);
    document.addEventListener("keyup", handleESC, false);

    return () => {
      document.removeEventListener("click", handleClickOut, false);
      document.removeEventListener("keyup", handleESC, false);
    };
  });
</script>

<div class="" class:ml-1={ml} bind:this={dropdownRef}>
  <button
    {disabled}
    class={`${clazz} button-container text-ellipsis overflow-hidden `}
    class:py-4={!small}
    class:py-3={small}
    class:opacity-50={disabled}
    class:cursor-not-allowed={disabled}
    class:primary-button-container={primary}
    class:secondary-button-container={secondary}
    on:click|preventDefault={() => (showDropdown = !showDropdown)}
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
  {#if showDropdown}
    <slot />
  {/if}
</div>
