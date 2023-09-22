<script lang='ts'>
	import { Ok, Err, Result } from 'ts-results';
	import { invoke } from '@tauri-apps/api/tauri';
	import { ProgressBar } from '@skeletonlabs/skeleton';

	// types
	type Entry = {
		filename: string;
		is_dir: boolean;
		size: number;
		created: number;
		modified: number;
	};

	type ResultItem = {
		Ok: Entry
		Err: unknown
	}

	type Errors = 'FS_PROBLEM' | 'FILE_FS_PROBLEM';

	// variables
	let entries: Entry[] = [];
	let inputStackBack: string[] = [];
	let inputStackForward: string[] = [];

	let inputValue = '';
	let inputRegistered = '';
	let loading = false;

	// reactivity
	$: isBackDisabled = inputStackBack.length < 1;
	$: isForwardDisabled = inputStackForward.length === 0;
	$: isInputValueDisabled = !inputValue.trim();

	// functions
	async function listFiles(path: string): Promise<ResultItem[]> {
		return await invoke('list_files', { path });
	}

	const goBack = async () => {
		if (inputStackBack && inputStackBack.length > 0) {
			let backInput = inputStackBack[inputStackBack.length - 1];

			let result = await getListFiles(backInput);

			if (result.ok) {
				let current = inputRegistered;

				inputStackBack.pop();

				// refresh array
				inputStackBack = [...inputStackBack];

				inputStackForward = [...inputStackForward, current];

				inputValue = backInput;
				inputRegistered = inputValue;

				entries = result.val;
			} else {
				console.log(result.val);
			}
		}
	};

	const goForward = async () => {
		if (inputStackForward && inputStackForward.length) {
			let forwardInput = inputStackForward[inputStackForward.length - 1];

			let result = await getListFiles(forwardInput);
			if (result.ok) {
				let current = inputRegistered;

				inputStackForward.pop();

				// refresh forward array
				inputStackForward = [...inputStackForward];

				inputStackBack = [...inputStackBack, current];
				inputValue = forwardInput;
				inputRegistered = inputValue;

				entries = result.val;
			} else {
				console.log(result.val);
			}
		}
	};

	const openEntry = async (entry) => {
		console.log(`clicked: ${entry.name}`);
		if (entry.is_dir) {
			let result = await getListFiles(entry.name);
			if (result.ok) {
				inputStackBack = [...inputStackBack, inputValue];
				inputValue = entry.name;
				inputRegistered = entry.name;

				entries = result.val;
			} else {
				console.log(result.val);
			}
		}
	};

	const parseDate = (date_in_sec_epoch) => {
		let d = new Date(0);
		d.setUTCSeconds(date_in_sec_epoch);
		return d.toDateString();
	};

	async function search() {
		let result = await getListFiles(inputValue);
		if (result.ok) {
			entries = result.val;
			inputRegistered = inputValue;
		} else {
			console.log(result.val);
		}
	}

	async function getListFiles(input: string): Promise<Result<Entry[], Errors>> {
		let result = null;
		loading = true;
		try {
			let files: ResultItem[] = await listFiles(input);

			result = files
				.filter((item: ResultItem) => Boolean(item.Ok))
				.map((item) => item.Ok)
				.map((item) => ({
					name: item.filename,
					is_dir: item.is_dir,
					size: item.size,
					created: parseDate(item.created),
					modified: parseDate(item.modified)
				}));

		} catch (e) {
			console.log('Error: ' + e);
			return new Err<Errors>('FS_PROBLEM');
		} finally {
			loading = false;
		}

		return Ok(result);
	}
</script>

<div class='flex flex-col gap-4 p-4'>
	<div class='flex flex-col card basis-1/4 gap-0'>
		<div class='flex flex-row flex-grow gap-3 p-3 h-full justify-center items-center'>
			<div class='flex flex-row gap-3 basis-1/12 justify-center'>
				<div>
					<button type='button' on:click={goBack} class='btn-icon variant-filled opacity-{isBackDisabled ? 50 : 100}'>
						<i class='fa-solid fa-arrow-left'></i>
					</button>
				</div>
				<div>
					<button
						type='button'
						on:click={goForward}
						class='btn-icon variant-filled opacity-{isForwardDisabled ? 50 : 100}'>
						<i class='fa-solid fa-arrow-right'></i>
					</button>
				</div>
			</div>
			<div class='basis-3/4'>
				<input class='input p-3' type='search' name='demo' bind:value={inputValue} placeholder='Search...' />
			</div>
			<div class='flex flex-row gap-3 basis-1/12 justify-center'>
				<button
					type='button'
					class='btn-icon variant-filled opacity-{isInputValueDisabled ? 50 : 100}'
					on:click={search}
					disabled={isInputValueDisabled}>
					<i class='fa-solid fa-skull'></i>
				</button>
			</div>
		</div>

		{#if loading}
			<div>
				<ProgressBar height='h-1' />
			</div>
		{/if}
	</div>
	<div class='card basis-3/4 p-4 flex flex-row'>
		<div class='table-container max-h-screen overflow-y-auto'>
			<!-- Native Table Element -->
			<table class='table table-hover table-fixed w-full'>
				<thead>
				<tr>
					<th class='w-3/5'>Name</th>
					<th>Created</th>
					<th>Modified</th>
				</tr>
				</thead>
				<tbody>
				{#each entries as entry}
					<tr on:dblclick={openEntry(entry)}>
						<td class='truncate ...'>
							<div class='flex flex-row justify-items-center items-center gap-3'>
								{#if entry.is_dir}
									<i class='fa fa-solid fa-folder-open' style='color:gold'></i>
								{:else}
									<i class='fa fa-solid fa-file'></i>
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
