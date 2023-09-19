<script lang="ts">
  import { emit, listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";

  import Sidebar from "./lib/Sidebar.svelte";
  import Topbar from "./lib/Topbar.svelte";
  import Tasks from "./lib/Tasks.svelte";

  let tasks: any[] = [];

  getTasks();
  async function getTasks() {
    tasks = await invoke("get_tasks_cmd");
  }

  const updateTasks = listen("task_update", (event) => {
    tasks = event.payload as any[];
  });
</script>

<main>
  <div>
    <h1>TODO <span>LIST</span></h1>
  </div>
  <Topbar />
  <Sidebar />
  {#key tasks}
    <Tasks />
  {/key}
</main>

<style>
  div {
    width: 100%;
    height: 100%;
    padding-left: 10%;
    background-color: var(--background-highlight);
    margin: 0;
    display: flex;
    align-items: center;
  }

  h1 span {
    color: var(--text-hover);
  }
</style>
