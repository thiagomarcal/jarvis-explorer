<script lang="ts">

    import {invoke} from "@tauri-apps/api/tauri";

    import {ProgressBar} from '@skeletonlabs/skeleton';


    async function listFiles(path: string) {
        return await invoke("list_files", {path})
    }

    const goBack = async () => {
        if (inputStackBack && inputStackBack.length > 1) {
            inputStackForward.push(inputStackBack.pop())
            console.log(`stack back: ${inputStackBack}`)

            console.log(`search for on back: ${inputStackBack}`)
            inputValue = inputStackBack[inputStackBack.length - 1];
            await doSearch()
        }
    }

    const goForward = async () => {
        if (inputStackForward && inputStackForward.length) {
            let current = inputStackForward.pop()
            inputStackBack.push(current)
            inputValue = current
            await doSearch()
        }
    }

    const openEntry = async (entry) => {
        console.log(`clicked: ${entry.name}`)
        if (entry.is_dir) {
            inputValue = entry.name
            inputStackBack.push(inputValue)
            await doSearch()
            console.log(`stack: ${inputStackBack}`)
        }
    }

    type Entry = {
        name: string;
        is_dir: boolean;
        size: number;
        created: string;
        modified: string;
    };

    const parseDate = (date_in_sec_epoch) => {
        let d = new Date(0);
        d.setUTCSeconds(date_in_sec_epoch)
        return d.toDateString()
    }

    async function search() {
        inputStackBack.push(inputValue)
        await doSearch()
    }

    async function doSearch() {
        loading = true
        let result = await listFiles(inputValue)
        console.log(result);

        entries = result
            .filter(item => Boolean(item.Ok))
            .map(item => item.Ok)
            .map(item => ({
                name: item.filename,
                is_dir: item.is_dir,
                size: item.size,
                created: parseDate(item.created.secs_since_epoch),
                modified: parseDate(item.modified.secs_since_epoch)
            }))

        loading = false
    }

    let entries: Entry[] = [];
    let inputStackBack: string[] = []
    let inputStackForward: string[] = []

    let inputValue = ""
    let loading = false


</script>

<div class="flex flex-col gap-4 p-4">
    <div class="flex flex-col card basis-1/4 gap-0">
        <div class="flex flex-row flex-grow gap-3 p-3 h-full justify-center items-center">
            <div class="flex flex-row gap-3 basis-1/12 justify-center">
                <div>
                    <button type="button" on:click={goBack} class="btn-icon variant-filled">
                        <i class="fa-solid fa-arrow-left"></i>
                    </button>
                </div>
                <div>
                    <button type="button" on:click={goForward} class="btn-icon variant-filled">
                        <i class="fa-solid fa-arrow-right"></i>
                    </button>
                </div>
            </div>
            <div class="basis-3/4">
                <input class="input p-3" type="search" name="demo" bind:value={inputValue}
                       placeholder="Search...">
            </div>
            <div class="flex flex-row gap-3 basis-1/12 justify-center">
                <button type="button" class="btn-icon variant-filled" on:click={search}>
                    <i class="fa-solid fa-skull"></i>
                </button>
            </div>

        </div>

        {#if loading}
            <div>
                <ProgressBar/>
            </div>
        {/if}

    </div>
    <div class="card basis-3/4 p-4 flex flex-row">
        <div class="table-container max-h-screen overflow-y-auto">
            <!-- Native Table Element -->
            <table class="table table-hover table-fixed w-full">
                <thead>
                <tr>
                    <th class="w-3/5">Name</th>
                    <th>Created</th>
                    <th>Modified</th>
                </tr>
                </thead>
                <tbody>
                {#each entries as entry}
                    <tr on:dblclick={openEntry(entry)}>
                        <td class="truncate ...">
                            <div class="flex flex-row justify-items-center items-center gap-3">
                                {#if entry.is_dir}
                                    <i class="fa fa-solid fa-folder-open" style="color:gold"></i>
                                {:else}
                                    <i class="fa fa-solid fa-file"></i>
                                {/if}
                                {entry.name}
                            </div>
                        </td>
                        <td>{entry.created}</td>
                        <td>{entry.modified}</td>
                    </tr>
                {/each}
                </tbody>
            </table>
        </div>
    </div>
</div>
