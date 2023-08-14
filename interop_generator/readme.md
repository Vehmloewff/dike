# dike/rusty_interop

Given a series of create manifests, this module generates the rust glue code the binds certain vm instructions to rust. Specifically, this
code is injected into the virtual machine at `interop_gen.rs`.

## Usage

```ts
import { CrateManifest, generateGlueCode } from './interop_generator/mod.ts'

let manifest1: CrateManifest = {
	name: 'some_crate',
	values: [
		{
			name: 'SomeValue',
			needsMemory: true,
			methods: [
				{
					name: 'foo',
					args: [
						{ type: 'external', crate: 'rusty_types', name: 'String' },
						{ type: 'external', crate: 'rusty_types', name: 'Number' },
					],
					returnType: { type: 'builtin', name: 'boolean' },
				},
			],
		},
	],
	functions: [
		{
			name: 'do_cool_stuff_with_some_value',
			args: [
				{ type: 'external', crate: 'rusty_types', name: 'String' },
				{ type: 'external', crate: 'rusty_types', name: 'Number' },
			],
			returnType: { type: 'builtin', name: 'boolean' },
		},
	],
}

let manifest2: CrateManifest = {
	name: 'rusty_types',
	values: [
		// ...
	],
	functions: [
		// ...
	],
}

const rustCode = generateGlueCode([manifest1, manifest2])
```
