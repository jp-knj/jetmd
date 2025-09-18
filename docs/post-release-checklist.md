# Post-release Checklist

1. **Verify published versions**
   - `npx npmview faster-md version`
   - `cargo search fmd-core --limit 1`
2. **Update documentation**
   - `STATUS.md`
   - `docs/compatibility.md`
   - `docs/RELEASE.md` (if steps changed)
3. **Communicate release**
   - Publish GitHub release (ensure draft contains changelog, attach dist-release artifacts)
   - Announce on internal channels / issue tracker
4. **Monitor**
   - npm download stats / error reports
   - crates.io metrics
5. **Follow-up Issues**
   - File tasks for any deferred cleanups or regressions found during release
