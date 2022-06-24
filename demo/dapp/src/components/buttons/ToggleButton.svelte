<script lang="ts">
  import "./button.scss";

  let clazz: string = "";
  export { clazz as class };
  export let primary: boolean = false;
  export let secondary: boolean = false;
  export let disabled: boolean = false;
  export let onClick: ((option: string) => void) | null;
  export let options: Array<string> = [];
  export let selected: string;

</script>

<div class="toggle-button-container">
  {#each options as option}
    <button
      {disabled}
      class={`${clazz} toggle-button text-ellipsis overflow-hidden py-4`}
      class:opacity-50={disabled}
      class:cursor-not-allowed={disabled}
      class:primary-button-container={primary}
      class:secondary-button-container={secondary}
      class:selected={selected == option}
      on:click|preventDefault={disabled
        ? null
        : () => {
            selected = option;
            onClick(option);
          }}
      aria-label={option}
      title={option}
    >
      {option}
    </button>
  {/each}
</div>
