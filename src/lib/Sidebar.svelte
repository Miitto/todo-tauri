<script lang="ts">
  import { emit, listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";

  let name = "";
  $: active = 0;
  let projects: any[] = [];

  getProjects();
  async function getProjects() {
    projects = await invoke("get_projects_cmd");
  }

  const updateTasks = listen("project_update", (event) => {
    projects = event.payload as any[];
  });

  async function deleteProject(ids: string) {
    console.log(ids);
    await invoke("delete_project", { ids });
  }

  async function newProject(event: any) {
    if (name != "") {
      await invoke("create_project", { name });
      name = "";
    }
  }

  function showNewProj() {
    let frm = document.getElementById("newProj");
    if (frm?.classList.contains("hidden")) {
      frm?.classList.remove("hidden");
      let inpt = document.getElementById("newProjInpt");
      inpt?.focus();
    } else frm?.classList.add("hidden");
  }

  getActive();
  async function getActive() {
    active = await invoke("get_projects_active");
    console.log(active);
  }

  async function setActive(ids: any) {
    await invoke("set_projects_active", { ids });
  }

  const updateActive = listen("active_update", (event) => {
    active = event.payload as any;
    console.log("Updated Active", active);
  });

  function isActive(idx: any) {
    return idx == active;
  }
</script>

<nav>
  <div id="title">
    <h5>Projects</h5>
    <button on:click={showNewProj}
      ><i class="fa-regular fa-square-plus" /></button
    >
  </div>
  <hr />
  {#key active}
    <div id="projHolder">
      {#each projects as project, idx}
        <!-- svelte-ignore missing-declaration -->
        <!-- svelte-ignore a11y-click-events-have-key-events -->
        <!-- svelte-ignore a11y-no-static-element-interactions -->
        <div
          class:active={idx == active}
          on:click={() => setActive(idx.toString())}
        >
          <p>{project.name}</p>
          <button on:click={() => deleteProject(idx.toString())}
            ><i class="fa-solid fa-trash-can" /></button
          >
        </div>
      {/each}
      <form id="newProj" class="hidden" on:submit|preventDefault={newProject}>
        <input
          id="newProjInpt"
          placeholder="New Project..."
          bind:value={name}
        />
        <input type="submit" hidden />
      </form>
    </div>
  {/key}
</nav>

<style>
  nav {
    width: 100%;
    height: 100%;
    background-color: var(--background-alt);
    padding: 1rem 0;
  }

  #title {
    display: flex;
    width: 100%;
    align-items: center;
    margin-bottom: 1rem;
  }

  #projHolder {
    display: flex;
    flex-direction: column;
    overflow-x: hidden;
    overflow-y: auto;
  }

  nav div div,
  nav div form {
    width: 100%;
    height: 3rem;
    padding-left: 1rem;
    display: flex;
    align-items: center;
  }

  .hidden {
    visibility: hidden;
  }

  input {
    max-width: calc(100% - 1rem);
  }

  h5 {
    padding-left: 1rem;
    font-size: 1.2rem;
    font-weight: normal;
    flex: 1 0 max-content;
  }

  div div p {
    flex: 1 0 max-content;
  }

  div button {
    margin-right: 1rem;
    padding: 5px;
    width: max-content;
    height: max-content;
    border: 0;
    box-shadow: none;
    background-color: unset;
  }

  i {
    box-shadow: none;
  }

  div button:hover {
    color: var(--text-hover);
    border: 0;
  }

  div div:hover,
  .active {
    background-color: var(--background);
  }
</style>
