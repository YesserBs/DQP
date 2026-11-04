"""Smoke tests for the user-facing distribution. Build the executable first."""
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
BINARY = Path(os.environ.get('DQP_BINARY', ROOT/'target/release'/('dqp.exe' if os.name=='nt' else 'dqp'))).resolve()

class DistributionTests(unittest.TestCase):
    def command(self, *args, **kwargs):
        return subprocess.run([str(BINARY), *map(str,args)], text=True, capture_output=True, **kwargs)

    def test_starter_scenario_and_report(self):
        with tempfile.TemporaryDirectory() as tmp:
            report=Path(tmp)/'resultat.json'
            run=self.command('run',ROOT/'starter/python.dqp.json','--output',report,input='a\nraison\na\n\nb\n\n')
            self.assertEqual(run.returncode,0,run.stderr)
            data=json.loads(report.read_text())
            self.assertEqual(data['result']['counters']['correct_count'],2)
            self.assertEqual(self.command('replay',ROOT/'starter/python.dqp.json',report).returncode,0)

    def test_documented_ai_example(self):
        text=(ROOT/'docs/v1/AI_GUIDE.md').read_text(encoding='utf-8')
        examples=[s for s in re.findall(r'```json\n(.*?)\n```',text,re.S) if '"dqp_version"' in s]
        self.assertEqual(len(examples),1)
        with tempfile.TemporaryDirectory() as tmp:
            source=Path(tmp)/'guide.dqp.json'
            source.write_text(examples[0],encoding='utf-8')
            result=self.command('check',source)
            self.assertEqual(result.returncode,0,result.stderr)

    def test_starter_schema_matches_reference(self):
        self.assertEqual((ROOT/'starter/.vscode/dqp.schema.json').read_bytes(),(ROOT/'schemas/dqp-1.0.schema.json').read_bytes())
        self.assertFalse((ROOT/'starter/src').exists())

    def test_tag_guard(self):
        for tag,expected in [('v0.1.2',0),('v9.9.9',1)]:
            result=subprocess.run([sys.executable,str(ROOT/'scripts/verify_tag.py'),tag],capture_output=True)
            self.assertEqual(result.returncode,expected)

    def test_versioned_documentation_link(self):
        result=subprocess.run([sys.executable,str(ROOT/'scripts/ai_link.py'),'https://github.com/example/dqp.git'],text=True,capture_output=True)
        self.assertEqual(result.returncode,0)
        self.assertIn('https://github.com/example/dqp/blob/v0.1.2/docs/v1/AI_GUIDE.md',result.stdout)

    @unittest.skipIf(os.name=='nt','Unix installer; Windows script requires PowerShell testing on Windows')
    def test_installer_and_no_overwrite(self):
        import shutil
        with tempfile.TemporaryDirectory() as tmp:
            tmp=Path(tmp)
            shutil.copy2(BINARY,tmp/'dqp')
            shutil.copy2(ROOT/'scripts/install.sh',tmp/'install.sh')
            destination=tmp/'user-bin'
            env={**os.environ,'DQP_INSTALL_DIR':str(destination)}
            first=subprocess.run(['sh',str(tmp/'install.sh')],env=env,text=True,capture_output=True)
            self.assertEqual(first.returncode,0,first.stderr)
            installed=(destination/'dqp').read_bytes()
            second=subprocess.run(['sh',str(tmp/'install.sh')],env=env,text=True,capture_output=True)
            self.assertEqual(second.returncode,1)
            self.assertEqual((destination/'dqp').read_bytes(),installed)

if __name__=='__main__': unittest.main()
