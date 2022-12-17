import axios from "axios";
import { readFileSync, writeFileSync, existsSync } from "fs";
import { resolve } from "path";
import { S3Client } from "@aws-sdk/client-s3";

const client = new S3Client({ region: "us-east-1" });

const EFS_PATH = "/tmp";

interface RawAlert {
  id: number;
  date_gmt: string;
  modified_gmt: string;
  slug: string;
  status: string;
  link: string;
  title: {
    rendered: string;
  };
}

interface Alert {
  id: number;
  date: Date;
  link: string;
  title: string;
}

const file_path = resolve(EFS_PATH, "prev.json");

export async function handler() {
  const rawData = await axios.get(process.env.URL as string);
  if (rawData.status < 200 || rawData.status >= 300 || rawData.data.length < 1)
    return;

  let data: Alert[] = rawData.data
    .filter((alert: RawAlert) => alert.status == "publish")
    .map(
      (alert: RawAlert): Alert => ({
        title: alert.title.rendered,
        id: alert.id,
        link: alert.link,
        date: new Date(alert.date_gmt),
      })
    );

  if (
    !existsSync(file_path) ||
    data != JSON.parse(readFileSync(file_path, "utf8"))
  ) {
    writeFileSync(file_path, JSON.stringify(data), { encoding: "utf8" });

    await Promise.all(
      data.map((alert) =>
        axios.post(process.env.WEBHOOK as string, {
          content: "" + alert.title,
        })
      )
    );
  }

  return { statusCode: 200 };
}
