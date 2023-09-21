<script lang="ts">
  import { emit, listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";

  import jQuery from "jquery";

  let tasks: any[] = [];

  getTasks();
  async function getTasks() {
    tasks = await invoke("get_tasks_cmd");
  }

  const updateTasks = listen("task_update", (event) => {
    tasks = event.payload as any[];
  });

  async function deleteTask(id: string) {
    console.log(id);
    await invoke("delete_task", { id });
  }

  async function toggleTask(event: any, id: string) {
    console.log(id);

    if (event.target.checked) await invoke("complete_task", { id });
    else await invoke("uncomplete_task", { id });
  }

  const updateActive = listen("active_update", (event) => {
    getTasks();
  });

  function showRenameTask(id: any) {
    jQuery(`.taskForm p.${id}`).addClass("hidden");
    jQuery(`.taskForm input.${id}[type=text]`)
      .removeClass("hidden")
      .trigger("focus")
      .on("focusout", function () {
        hideRenameTask(id);
      });
  }

  function toggleRenameTask(id: any) {
    if (jQuery(`.taskForm p.${id}`).hasClass("hidden")) {
      hideRenameTask(id);
    } else {
      showRenameTask(id);
    }
  }

  function hideRenameTask(id: any) {
    jQuery(`.taskForm p.${id}`).removeClass("hidden");
    jQuery(`.taskForm input.${id}[type=text]`)
      .addClass("hidden")
      .off("focusout");
  }

  async function renameTask(event: any, ids: string) {
    let name: string = event.target.elements["name"].value;
    await invoke("rename_task", { ids, name });
    hideRenameTask(ids);
  }
</script>

<div>
  <form class="taskForm">
    {#each tasks as task}
      <form
        on:submit|preventDefault={(event) =>
          renameTask(event, task.id.toString())}
      >
        <p class={task.id}>{task.name}</p>
        <input
          class="{task.id} hidden"
          type="text"
          placeholder={task.name}
          name="name"
        />
        <input type="submit" hidden />
      </form>
      <input
        name={task.name}
        bind:checked={task.done}
        on:change={(event) => toggleTask(event, task.id.toString())}
        type="checkbox"
      />
      <button
        class={task.id}
        on:click|stopPropagation|preventDefault={() =>
          toggleRenameTask(task.id.toString())}
        ><i class="fa-solid fa-pen" /></button
      >
      <button on:click={() => deleteTask(task.id.toString())}
        ><i class="fa-solid fa-trash-can" /></button
      >
    {/each}
  </form>
</div>

<style>
  div {
    width: 100%;
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
    padding-bottom: 1.5rem;
  }

  .hidden {
    visibility: hidden;
    display: none;
  }

  .taskForm {
    overflow-x: hidden;
    overflow-y: scroll;
    display: grid;
    width: 100%;
    flex: 0 0 100%;
    background-color: var(--background);
    margin: 0;
    align-items: center;
    grid-template-columns: 1fr 2rem 2rem 2rem;
    gap: 10px;
    grid-template-rows: 3rem repeat(auto-fit, 5rem);
    padding: 1rem;
    max-height: 100%;

    grid-template-areas:
      "header header header"
      "main main main";
  }

  input[type="checkbox"] {
    width: 1.25rem;
    height: 5rem;
    border: 0;
    box-shadow: none;
    color: #ffeeee;
  }

  input[type="text"] {
    width: 100%;
  }

  button {
    margin-right: 1rem;
    padding: 5px;
    width: max-content;
    height: max-content;
    border: 0;
    box-shadow: none;
    background-color: unset;
  }

  button:hover {
    color: var(--text-hover);
    border: 0;
  }
</style>
