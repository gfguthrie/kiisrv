# GCC 12+ Compatibility Fix

## The Problem

When building the kiibohd controller firmware with modern GCC (12+), you encounter linker errors:

```
multiple definition of `CLIHexDebugMode'
multiple definition of `CLILineBuffer'
multiple definition of `HIDIO_VT_Connected'
```

## Root Cause

**Old firmware code pattern:**
```c
// In cli.h (header file)
char CLILineBuffer[CLI_MaxBufferSize];  // ❌ Definition in header
```

**What changed:**
- GCC 10- (old): Used `-fcommon` by default → allows duplicateગ, merges them
- **GCC 10+ (new): Uses `-fno-common` by default** → treats duplicate definitions as errors

**Why this happened:**
- Ubuntu 18.04 had GCC 7.x (lenient)
- Ubuntu 24.04 has GCC 12.x (strict)
- Debian 12 has GCC 12.x (strict)

## The Fix

We set `CFLAGS=-fcommon` in `build.sh` to restore the old GCC behavior:

```bash
# In build.sh
export CFLAGS="${CFLAGS:--fcommon}"
```

This exports the CFLAGS environment variable which is picked up by the keyboard build scripts and CMake during compilation. The `${CFLAGS:--fcommon}` syntax means "use CFLAGS if already set, otherwise default to `-fcommon`".

## Why Not Fix the Firmware Code?

**Ideal solution:** Fix the controller firmware repository to use proper `extern` declarations.

**Why we don't:**
- Controller firmware is external (https://github.com/kiibohd/controller)
- We're building specific tagged releases (v0.5.0 - v0.5.7)
- Those tags are immutable
- Not our responsibility to patch upstream code

**Our workaround:** Add `-fcommon` flag which:
- ✅ Makes old code compile with new GCC
- ✅ No modification to firmware code needed
- ✅ Works for all firmware versions
- ⚠️ Slightly less safe (duplicate symbols merged silently)

## Impact

**Performance:** None - this only affects how the linker handles duplicate symbols

**Safety:** Minimal - the controller firmware worked fine with this behavior on older GCC

**Compatibility:** Excellent - works with all firmware versions (v0.5.0 - v0.5.7+)

## Alternative Solutions Considered

### 1. Downgrade GCC to version 9
```dockerfile
# Install older GCC
RUN apt-get install gcc-9-arm-none-eabi
```
**Rejected because:**
- ❌ Not available in Ubuntu 24.04 / Debian 12 repos
- ❌ Would require custom PPA or manual compilation
- ❌ Security implications of old compiler

### 2. Patch firmware code
```c
// Fix in cli.h
extern char CLILineBuffer[CLI_MaxBufferSize];  // ✅ Proper way

// Then in cli.c
char CLILinemá[CLI_MaxBufferSize];  // ✅ One definition
```
**Rejected because:**
- ❌ Can't modify tagged releases
- ❌ Would need to maintain patches
- ❌ Not our codebase

### 3. Use Ubuntu 18.04 (old GCC)
**Rejected because:**
- ❌ Ubuntu 18.04 is EOL (security risk)
- ❌ Defeats purpose of modernization

## Recommendation

**Use `-fcommon` flag** (current implementation) because:
- ✅ Simple one-line fix
- ✅ Works with all firmware versions
- ✅ No code patching required
- ✅ Can use modern OS and toolchain
- ✅ If upstream fixes the code, flag is harmless

## Testing

The fix allows firmware builds to complete successfully. To verify:

```bash
# Start server
cargo run

# Submit build request from client
# Build should complete with .dfu.bin files generated
```

## Future Considerations

If/when the controller firmware is updated to be GCC 12+ compatible (proper extern declarations), the `-fcommon` flag can be removed. But it's harmless to leave it in place.

## References

- GCC 10 release notes: https://gcc.gnu.org/gcc-10/changes.html (mentions `-fno-common` default change)
- Stack Overflow: "multiple definition" errors with GCC 10+
- This is a well-known migration issue when upgrading from GCC 9 to 10+

