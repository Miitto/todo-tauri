<script lang="ts">
  import { emit, listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";

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
</script>

<div>
  <div id="title">
    <h3 style="flex: 1 0 max-content">Task Name</h3>
    <h3 style="flex: 0 0 5rem; margin-right: 2rem">Done</h3>
    <h3 style="flex: 0 0 5rem">Delete</h3>
  </div>
  <div>
    {#each tasks as task, idx}
      <label for={task.name}>{task.name}</label>
      <input
        name={task.name}
        bind:checked={task.done}
        on:change={(event) => toggleTask(event, idx.toString())}
        type="checkbox"
      />
      <button on:click={() => deleteTask(idx.toString())}
        ><i class="fa-solid fa-trash-can" /></button
      >
    {/each}
  </div>
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

  #title {
    width: 100%;
    display: flex;
    height: 3rem;
    overflow: hidden;
    justify-content: left;
    flex-direction: row;
    flex: 0 0 3rem;
  }

  div div {
    overflow-x: hidden;
    overflow-y: scroll;
    display: grid;
    width: 100%;
    flex: 0 0 100%;
    background-color: var(--background);
    margin: 0;
    align-items: center;
    grid-template-columns: 1fr 5.8rem 4.5rem;
    gap: 10px;
    grid-template-rows: 3rem repeat(auto-fit, 5rem);
    padding: 1rem;
    max-height: 100%;

    grid-template-areas:
      "header header header"
      "main main main";
  }

  h3 {
    font-weight: normal;
  }

  input[type="checkbox"] {
    width: 1.25rem;
    height: 5rem;
    border: 0;
    box-shadow: none;
    color: #ffeeee;
  }

  button {
    background-color: var(--button);
  }
</style>
