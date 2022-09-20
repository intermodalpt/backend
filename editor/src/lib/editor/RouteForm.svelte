<script>
  import {writable} from "svelte/store";
  import {api_server, token} from "../../settings.js";

  export let selectedRoute = writable({
    code: undefined,
    name: undefined,
    mainSubroute: undefined,
    subroutes: [],
  });


  let code;
  let name;
  let circular;
  let active;
  let mainSubrouteIndex;
  let routeChanged = false;
  let subroutes = [];

  if (selectedRoute !== undefined) {
    selectedRoute.subscribe((route) => {
      if (route) {
        code = route.code;
        name = route.name;
        circular = route.circular || false;
        active = route.active;
        mainSubrouteIndex = route.main_subroute;
        routeChanged = false
        subroutes = [...route.subroutes];
      }
    });
  }

  function addSubroute() {
    subroutes.push({
      'id': undefined,
      'flag': "",
      'circular': false,
      'changed': true,
    });
    subroutes = subroutes;
  }

  function saveRoute() {
    let routeId = $selectedRoute.id;
    let savedCount = 0;
    for (const subroute of subroutes) {
      if (!subroute.changed && subroute.id) {
        continue;
      }
      if (subroute.id) {
        fetch(`${api_server}/api/routes/${routeId}/${subroute.id}`, {
          method: "PATCH",
          headers: {
            "Content-Type": "application/json",
            authorization: `Bearer ${$token}`,
          },
          body: JSON.stringify({
            flag: subroute.flag,
            circular: subroute.circular,
          })
        })
            .catch((e) => {
              alert(`Error patching a route: ${JSON.stringify(subroute)}`);
            })
            .then((res) => {
              switch (res.status) {
                case 200:
                  subroute.changed = false;
                  savedCount += 1;
                  break;
                default:
                  alert("One request might have succeeded, or not");
              }
            });
      } else {
        fetch(`${api_server}/api/routes/${$selectedRoute.id}/create_subroute`, {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            authorization: `Bearer ${$token}`,
          },
          body: JSON.stringify({
            flag: subroute.flag,
            circular: subroute.circular,
          })
        })
            .catch((e) => {
              alert(`Error creating a route: ${JSON.stringify(subroute)}`);
            })
            .then((res) => {
              switch (res.status) {
                case 200:
                  subroute.changed = false;
                  savedCount += 1;
                  res.json().then((id) => {
                    subroute.id = id
                  })
                  break;
                default:
                  alert("One request might have succeeded, or not");
              }
            });
      }
    }

    if (savedCount > 0) {
      alert(`Saved ${savedCount} changes`);
    }

    if (routeChanged) {
      let mainSubrouteId = subroutes[mainSubrouteIndex] ? subroutes[mainSubrouteIndex].id : null;
      fetch(`${api_server}/api/routes/${routeId}`, {
        method: "PATCH",
        headers: {
          "Content-Type": "application/json",
          authorization: `Bearer ${$token}`,
        },
        body: JSON.stringify({
          code: code,
          name: name,
          circular: circular,
          main_subroute: mainSubrouteId,
          // FIXME
          operator: 1,
          badge_text: "fa3250",
          badge_bg: "000000",
          active: true,
        })
      })
          .catch((e) => {
            alert(`Error patching a route: ${JSON.stringify($selectedRoute)}`);
          })
          .then((res) => {
            switch (res.status) {
              case 200:
                alert("Route saved");
                routeChanged = false;
                break;
              default:
                alert("One request might have succeeded, or not");
            }
            return res.json()
          });
    }
  }

  function delSubroute(index) {
    if (subroutes[index].id === undefined) {
      subroutes.splice(index, 1);
    } else if (confirm(`Do you really really want to delete ${subroutes[index].flag}? This will fail if it has stops or departures.`)) {
      fetch(`${api_server}/api/routes/${$selectedRoute.id}/${subroutes[index].id}`, {
        method: "DELETE",
        headers: {
          "Content-Type": "application/json",
          authorization: `Bearer ${$token}`,
        },
      })
          .catch((e) => {
            alert("Error deleting");
            console.log(e);
          })
          .then((res) => {
            switch (res.status) {
              case 200:
                alert("Done.");
                subroutes.splice(index, 1);
                subroutes = subroutes;
                break;
              case 424:
                alert("You need to delete the stops before you delete the subroute");
                break;
              default:
                alert("The request might have succeeded, or not");
            }
          });

    }
    subroutes = subroutes;
  }


  function delRoute() {
    if (!($selectedRoute && $selectedRoute.id)) {
      alert("This route cannot be deleted");
    }

    if (confirm(`Do you really really want to delete ${$selectedRoute.code}? This will fail if it has subroutes.`)) {
      fetch(`${api_server}/api/routes/${$selectedRoute.id}`, {
        method: "DELETE",
        headers: {
          "Content-Type": "application/json",
          authorization: `Bearer ${$token}`,
        },
      })
          .catch((e) => {
            alert("Error deleting");
            console.log(e);
          })
          .then((res) => {
            switch (res.status) {
              case 200:
                alert("Done. Please close this route window as I was too lazy to make it close by itself");
                break;
              case 424:
                alert("You need to delete the subroutes before you delete the route");
                break;
              default:
                alert("The request might have succeeded, or not");
            }
          });
    }
  }
</script>


<div>
  <div class="flex gap-1 p-2 overflow-visible w-full">
    <div class="form-control">
      <label class="input-group">
        <span>Code</span>
        <input
            type="text"
            bind:value={code}
            on:change={() => routeChanged = true}
            class="input input-bordered w-24 input-md" />
      </label>
    </div>
    <div class="form-control w-full">
      <label class="input-group">
        <span>Name</span>
        <input
            type="text"
            bind:value={name}
            on:change={() => routeChanged = true}
            class="input input-bordered w-full input-md" />
      </label>
    </div>
    <div class="form-control">
      <label class="input-group">
        <span>Circular</span>
        <input
            type="checkbox"
            bind:checked={circular}
            on:change={() => routeChanged = true}
            class="input" />
      </label>
    </div>
    <div class="form-control">
      <label class="input-group">
        <span>Active</span>
        <input
            type="checkbox"
            bind:checked={active}
            on:change={() => routeChanged = true}
            class="input" />
      </label>
    </div>
  </div>
</div>
<table class="table table-zebra table-compact w-full  mb-4">
  <thead>
  <tr>
    <th>Main</th>
    <th class="w-full">Flag</th>
    <th class="w-full">Circular</th>
    <th>
      <input type="button" class="btn btn-success btn-xs" value="Add" on:mouseup={addSubroute} />
    </th>
  </tr>
  </thead>
  <tbody>
  {#each subroutes as subroute, index}
    <tr>
      <td><input type=radio bind:group={mainSubrouteIndex} name="main-route" value={index}></td>
      <td>
        <input
            class="input input-bordered w-full"
            type="text"
            bind:value={subroute.flag}
            on:change={() => subroute.changed = true} />
      </td>
      <td>
        <input
            class="input input-bordered"
            type="checkbox"
            bind:checked={subroute.circular}
            on:change={() => subroute.changed = true} />
      </td>
      <td>
        <input type="button" class="btn btn-error btn-xs" value="Del" on:mouseup={() =>{delSubroute(index)}} />
      </td>
    </tr>
  {/each}
  </tbody>
</table>
<div class="flex justify-between">
  <button class="btn btn-error" on:mouseup={delRoute} disabled="{subroutes.length !== 0}">Delete</button>
  <button
      class="btn btn-primary"
      on:mouseup={saveRoute}
      disabled="{!routeChanged && subroutes.findIndex((sr) => {return sr.changed}) === -1}">
    Save
  </button>
</div>
