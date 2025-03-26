# 雑記

## Dockerとか

`compose.yml`(or `Dockerfile`)は最低2つずつ存在する。
`./devcontainer`以下はvscodeでの開発で使用し、`.(project root)`以下は本番環境で使用させる。

`devcontainer.json`内の`image`はコメントアウトにあるとおり、`Dockerfile`, `compose.yml`から読み込み可能, 公式imageでもいい
あと、gui生成の場合indentにtabが使用されるがspaceに変更してもいいっぽい。src微妙。


本番環境用のファイルたちは**動作未確認**。

`.devcontainer/devcontainer.json`の`remoteUser`のコメントアウトを外し`root`にすると、安全でないディレクトリとしてワークスペースが認識されgitが使えなくなる。config変更することで対処可能
