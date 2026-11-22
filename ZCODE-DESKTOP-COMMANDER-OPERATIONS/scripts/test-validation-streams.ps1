<#
.SYNOPSIS
Deterministic non-product acceptance probe for raw full-output recording.
.DESCRIPTION
Emits exactly 8,192 ASCII 'A' bytes to stdout and 2,048 ASCII 'B' bytes to
stderr, without shell text formatting or profiles. No provider/product effects.
#>
$ErrorActionPreference='Stop'
$outBytes=[byte[]]::new(8192)
$errBytes=[byte[]]::new(2048)
for($i=0;$i -lt $outBytes.Length;$i++){$outBytes[$i]=65}
for($i=0;$i -lt $errBytes.Length;$i++){$errBytes[$i]=66}
$out=[Console]::OpenStandardOutput()
$err=[Console]::OpenStandardError()
$out.Write($outBytes,0,$outBytes.Length)
$err.Write($errBytes,0,$errBytes.Length)
$out.Flush()
$err.Flush()
