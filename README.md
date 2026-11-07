# Domain List to SRS

This is a converter that transforms [domain-list-community](https://github.com/v2fly/domain-list-community) rules into sing-box RuleSet v3 SRS format. It uses my own implementation to generate SRS. When the upstream implementation changes, the RuleSet produced by this converter may become incompatible. No warranty is provided; use at your own risk. A GitHub Action runs automatically every day at 00:00Z, and the artifacts are pushed to the [generate](https://github.com/skymkmk/domain-list-to-srs/tree/generate) branch.

## Acknowledgments

- [v2fly/domain-list-community and its contributors](https://github.com/v2fly/domain-list-community)
- [sing-box and its contributors](https://github.com/SagerNet/sing-box)
- [sing and its contributors](https://github.com/SagerNet/sing)
- [世界](https://github.com/nekohasekai)

## License

The converter itself is released under AGPLv3. The generated rule sets (artifacts) are distributed under the upstream MIT license.

Domain List to SRS, a converter that transforms domain-list-community rules into SRS.

Copyright (C) 2025 skymkmk

This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License version 3 as published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.