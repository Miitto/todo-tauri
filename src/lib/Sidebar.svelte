<script lang="ts">
  import { emit, listen } from "@tauri-apps/api/event";
  import { invoke } from "@tauri-apps/api/tauri";

  import jQuery from "jquery";

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
    active = await invoke("get_active_cmd");
    console.log(active);
  }

  async function setActive(ids: any) {
    await invoke("set_projects_active", { ids });
    console.log(ids);
  }

  const updateActive = listen("active_update", (event) => {
    active = event.payload as any;
    console.log("Updated Active", active);
  });

  function isActive(idx: any) {
    return idx == active;
  }

  function toggleRenameProject(id: any) {
    if (
      jQuery(`.projForm p.${id}, .projForm button.${id}`).hasClass("hidden")
    ) {
      hideRenameProject(id);
    } else {
      showRenameProject(id);
    }
  }

  function showRenameProject(id: any) {
    jQuery(`.projForm p.${id}, .projForm button.${id}`).addClass("hidden");
    jQuery(`.projForm input.${id}[type=text]`)
      .removeClass("hidden")
      .trigger("focus")
      .on("focusout", function () {
        hideRenameProject(id);
      });
  }

  function hideRenameProject(id: any) {
    jQuery(`.projForm p.${id}, .projForm button.${id}`).removeClass("hidden");
    jQuery(`.projForm input.${id}[type=text]`)
      .addClass("hidden")
      .off("focusout");
  }

  async function renameProject(event: any, ids: string) {
    let name: string = event.target.elements["name"].value;
    await invoke("rename_project", { ids, name });
    hideRenameProject(ids);
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
    {#key projects}
      <div id="projHolder">
        {#each projects as project}
          <!-- svelte-ignore missing-declaration -->
          <!-- svelte-ignore a11y-click-events-have-key-events -->
          <!-- svelte-ignore a11y-no-static-element-interactions -->
          <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
          <form
            class="projForm"
            class:active={project.id == active}
            on:click={() => setActive(project.id.toString())}
            on:submit|preventDefault={(event) =>
              renameProject(event, project.id.toString())}
          >
            <p class={project.id}>{project.name}</p>
            <input
              class="{project.id} hidden"
              type="text"
              placeholder={project.name}
              name="name"
            />
            <input type="submit" hidden />
            <button
              class={project.id}
              on:click|stopPropagation|preventDefault={() =>
                showRenameProject(project.id.toString())}
              ><i class="fa-solid fa-pen" /></button
            >
            <button
              on:click|stopPropagation={() =>
                deleteProject(project.id.toString())}
              ><i class="fa-solid fa-trash-can" /></button
            >
          </form>
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
    max-width: 100%;
  }

  #projHolder input[type="text"] {
    display: flex;
    flex-direction: column;
    overflow-x: hidden;
    overflow-y: auto;
    flex: 1 1 auto;
    max-width: calc(100% - 1rem);
  }

  .projForm,
  nav div form {
    width: 100%;
    max-width: 100%;
    height: 3rem;
    display: flex;
    align-items: center;
  }

  .hidden {
    visibility: hidden;
    display: none;
  }

  input {
    max-width: calc(100% - 1rem);
    flex: 1 1 auto;
  }

  h5 {
    padding-left: 1rem;
    font-size: 1.2rem;
    font-weight: normal;
    flex: 1 0 max-content;
  }

  .projForm p {
    flex: 1 0 max-content;
    padding-left: 1rem;
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

  .projForm:hover,
  .active {
    background-color: var(--background);
  }
</style>
