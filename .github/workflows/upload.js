const fs = require("fs").promises;
const crypto = require("crypto");
const zlib = require("zlib");
const tar = require("tar-fs");

function targz(files) {
  return new Promise((resolve, reject) => {
    console.log("targz files: ", files[0].name, files[0]);

    const tarStream = tar.pack();

    for (const file of files) {
      tarStream.entry({ name: file.name }, file.data);
    }

    tarStream.finalize();

    const gzip = zlib.createGzip();

    const chunks = [];
    tarStream
      .pipe(gzip)
      .on("data", (chunk) => {
        chunks.push(chunk);
      })
      .on("end", () => {
        const buffer = Buffer.concat(chunks);
        console.log(buffer);
        resolve(buffer);
      })
      .on("error", reject);
  });
}

module.exports = async ({ github, context }) => {
  const {
    repo: { owner, repo },
    sha,
  } = context;
  console.log(process.env.GITHUB_REF);
  const release = await github.rest.repos.getReleaseByTag({
    owner,
    repo,
    tag: process.env.GITHUB_REF.replace("refs/tags/", ""),
  });
  console.log("release id: ", release.data.id);
  const VERSION = process.env.GITHUB_REF_NAME;
  const release_id = release.data.id;

  const compiled_extensions = [
    {
      name: "fastrand0.so",
      path: "sqlite-fastrand-ubuntu/fastrand0.so",
      asset_name: `sqlite-fastrand-${VERSION}-ubuntu-x86_64.tar.gz`,
    },
    {
      name: "fastrand0.dylib",
      path: "sqlite-fastrand-macos/fastrand0.dylib",
      asset_name: `sqlite-fastrand-${VERSION}-macos-x86_64.tar.gz`,
    },
    {
      name: "fastrand0.dylib",
      path: "sqlite-fastrand-macos-arm/fastrand0.dylib",
      asset_name: `sqlite-fastrand-${VERSION}-macos-arm64.tar.gz`,
    },
    {
      name: "fastrand0.dll",
      path: "sqlite-fastrand-windows/fastrand0.dll",
      asset_name: `sqlite-fastrand-${VERSION}-windows-x86_64.tar.gz`,
    },
  ];

  const extension_assets = await Promise.all(
    compiled_extensions.map(async (d) => {
      const extensionContents = await fs.readFile(d.path);
      const ext_sha256 = crypto
        .createHash("sha256")
        .update(extensionContents)
        .digest("hex");
      const tar = await targz([{ name: d.name, data: extensionContents }]);

      const tar_md5 = crypto.createHash("md5").update(tar).digest("base64");
      const tar_sha256 = crypto.createHash("sha256").update(tar).digest("hex");
      return {
        ext_sha256,
        tar_md5,
        tar_sha256,
        tar,
        asset_name: d.asset_name,
      };
    })
  );
  console.log("assets length: ", extension_assets.length);
  const checksum = {
    extensions: Object.fromEntries(
      extension_assets.map((d) => [
        d.asset_name,
        {
          asset_sha265: d.tar_sha256,
          asset_md5: d.tar_md5,
          extension_sha256: d.ext_sha256,
        },
      ])
    ),
  };
  console.log("checksum", checksum);

  await github.rest.repos.uploadReleaseAsset({
    owner,
    repo,
    release_id,
    name: "spm.json",
    data: JSON.stringify(checksum),
  });

  await Promise.all(
    extension_assets.map(async (d) => {
      console.log("uploading ", d.asset_name);
      await github.rest.repos.uploadReleaseAsset({
        owner,
        repo,
        release_id,
        name: d.asset_name,
        data: d.tar,
      });
    })
  );
  return;
};
