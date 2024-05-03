import templates from '../data/templates.json' assert { type: 'json' };
import { Bench, hrtimeNow } from 'tinybench';
import { vbml } from '@vestaboard/vbml';
import { writeFile } from 'fs/promises';

const bench = new Bench({ time: 5_000, warmupTime: 500, now: hrtimeNow });
for (const template of templates) bench.add(template.name, () => vbml.parse(template.data));

bench.addEventListener('warmup', () => console.log(`warming up benchmark`));
bench.addEventListener('start', () => console.log(`starting benchmark`));
bench.addEventListener('cycle', (e) =>
  console.log(`${e.task.name} completed ${e.task.runs} runs in ${e.task.result.totalTime}ms`)
);

await bench.warmup();
await bench.run();

await writeFile(
  '../out/js.json',
  JSON.stringify(
    bench.results.map(({ samples, ...data }, i) => ({ name: templates[i].name, ...data })),
    null,
    2
  )
);

console.table(bench.table());
