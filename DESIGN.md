# Design

Demens uses Terraformer3D as a static lib. It is possible to just use Terraformer3D directly (on the iced-rs engine).

## TRX File Format

A text or binary format for Terraformer3D. Each file stores a scene with child nodes. A node can be a scene itself or a leaf node, such as a model/mesh/etc.

- Materials and textures are stored within a texture/material node, which itself is a subnode of a mesh or model

### Text TRX

Inspired by YAML, reliant on spacing and layout

```yml
META
<number of meshes>:
<number of nodes>:
<depth of tree>:
<width of tree>:

SCENE
<root scene>
<scene 0>:
    <subtree 0, scenes, nodes>
<scene 1>:
    <subtree 1, scenes, nodes>
...
<scene n-1>:
    <subtree n-1, scenes, nodes>
```

### Binary TRX

Think of it like normal binary files like ELF and BLEND

```yml
<magic number:TRX><number of meshes><number of nodes><depth of tree><width of tree>
<scene 0><subtree 0, scenes, nodes..>
<scene 1><subtree 1, scenes, nodes..>
...
<scene n-1><subtree n-1, scenes, nodes..>
```
