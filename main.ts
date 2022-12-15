import axios from "axios";
import { readFileSync, writeFileSync, existsSync } from "fs";
import { createHash } from "crypto";
import * as dotenv from "dotenv";
dotenv.config();

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

axios.get("https://www.apsva.us/wp-json/wp/v2/mat_alert").then((rawData) => {
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

  const hash = createHash("sha256").update(JSON.stringify(data)).digest("hex");
  if (!existsSync("hash.txt") || hash != readFileSync("hash.txt", "utf8")) {
    writeFileSync("hash.txt", hash, { encoding: "utf8" });

    data.forEach((alert) => {
      axios.post(process.env.WEBHOOK_URL, { content: alert.title });
    });
  }
});
