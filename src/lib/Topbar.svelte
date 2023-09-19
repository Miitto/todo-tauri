<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  let projectTitle = "";

  async function getProjectTitle() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    projectTitle = await invoke("get_project_title");
  }

  getProjectTitle();

  async function newTask(event: any) {
    if (name != "") {
      await invoke("create_task", { name });
      name = "";
    }
  }
</script>

<form on:submit|preventDefault={newTask}>
  <h1>{projectTitle}</h1>
  <input placeholder="New Task..." bind:value={name} />
  <input type="submit" hidden />
</form>

<style>
  form {
    width: 100%;
    height: 100%;
    background-color: var(--background-alt);
    display: flex;
    justify-content: center;
    align-items: center;
    gap: 2rem;
  }

  form input {
    width: 60%;
  }
</style>
